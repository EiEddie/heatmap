use std::{env, io};

use chrono::{Datelike, Local, Months, NaiveDate};
use clap::Parser;
use heatmap::*;

#[derive(Parser, Debug)]
#[command(version, about = "Heatmap about dates for a given data source.")]
struct Cli {
	/// Date range, like:
	///   <EMPTY>                   : from first day 2 months ago to today,
	///                               only if has other opts
	///   "20220101-20231231"       : from 1 Jan, 2022 to 31 Dec, 2023
	///   "_-20231231"              : from 1 Jan the first year which have data to 31 Dec, 2023
	///   "20220101-_" or "20220101": from 1 Jan, 2022 to today
	#[clap(verbatim_doc_comment)]
	range: Option<String>,

	/// Print given year
	#[arg(short = 'y', long = "year")]
	year: Option<i32>,

	/// Source of data
	#[arg(short = 's', long = "src")]
	data_src: Option<String>,
}

fn run() -> error::Result<()> {
	let cli = Cli::parse();
	#[cfg(debug_assertions)]
	dbg!(&cli);

	// 从数据源获取数据
	// 从命令行指定的优先级大于从环境变量获取的
	let data_path = if let Some(src) = cli.data_src {
		src
	} else if let Ok(src) = env::var("DATA_PATH") {
		src
	} else {
		return Err(error::Error::NoSourceOfData);
	};
	// TODO: 判断数据来源类型

	// 连接数据库
	let db = import::sqlite::Database::new(&data_path)?;

	// 导入数据
	let data = stat::YearData::from(db)?;

	// 解析传入的日期区间
	let (from, to) = if let Some(range) = cli.range {
		// 根据范围解析
		let range: Vec<_> = range.split('-').collect();

		let from = range.get(0).ok_or("Range error, left is empty")?.to_owned();
		let to = range.get(1).map_or("_", |x| x.to_owned());

		data.parse_range(from, to, "%Y%m%d")?
	} else if let Some(year) = cli.year {
		// 根据年份解析
		// 从 1月1日 到 12月31日
		(NaiveDate::from_ymd_opt(year, 1, 1).unwrap(),
		 NaiveDate::from_ymd_opt(year, 12, 31).unwrap())
	} else {
		// 空的输入参数
		// 从 2个月前的1号 到 今天
		let today = Local::now().date_naive();
		let from =
			NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap() - Months::new(2);
		(from, today)
	};

	data.print_range(from, to)?;

	Ok(())
}

fn main() {
	if let Err(err) = run() {
		let mut stderr = io::stderr();
		error::error_handler(&err, &mut stderr);
	}
}
