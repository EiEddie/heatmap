use std::{env, io};

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
	range: Option<String>,

	/// Print given year
	#[arg(short = 'y', long = "year")]
	year: Option<i32>,

	/// Source of data
	#[arg(short = 's', long = "src")]
	data_src: Option<String>,
}

fn run() -> error::Result<()> {
	let data_path = env::var("DATA_PATH").map_err(|_| error::Error::NoSourceOfData)?;

	let db = import::sqlite::Database::new(&data_path)?;

	stat::YearData::from(db)?.print_auto()?;

	Ok(())
}

fn main() {
	if let Err(err) = run() {
		let mut stderr = io::stderr();
		error::error_handler(&err, &mut stderr);
	}
}
