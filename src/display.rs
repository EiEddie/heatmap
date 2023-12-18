use std::fmt::{self, Display};
use std::io;

use chrono::{Datelike, NaiveDate};
use crossterm::style::Stylize;

use crate::error::{Error, Result};
use crate::stat::{DaysData, YearData};

/// 用字符及其颜色反映给定值的程度
///
/// 必须保证宽度相等
///
/// 若传入 `u32::MAX`, 给出一个空白, 这样可以保证所有显示的宽度都相等
fn levels(cnt: u32) -> String {
	return match cnt {
		u32::MAX => " ".to_string(),
		0 => ".".dark_grey().to_string(),
		1 => "+".green().to_string(),
		2 => "%".yellow().to_string(),
		3 => "@".red().to_string(),
		4..=u32::MAX => "#".on_magenta().to_string(),
	};
}

impl Display for DaysData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// 每一个月第一天在这一年的序数
		// 从 0 计数
		let ord_mon: Vec<_> =
			(1 as u32..=12).map(|m| NaiveDate::from_ymd_opt(self.year, m, 1).unwrap().ordinal0())
			               .collect();

		// 这一年的第一天是星期几
		// 从 0 计数
		let fst_weekday = NaiveDate::from_yo_opt(self.year, 1).unwrap()
		                                                      .weekday()
		                                                      .number_from_monday()
		                  - 1;

		let mut data_iter = self.data.iter();
		let mut ord_mon_iter = ord_mon.iter().enumerate();

		let mut data_val = data_iter.next();
		let mut ord_mon_val = ord_mon_iter.next();

		// 一年最多 54 周
		'week: for week in 0..54 {
			// 这一周是否包含下一个月的第一天
			// 即两个月的交界处是否是这一周
			let mut is_next_mon_w = false;

			// 若这一周包含下一个月的第一天
			// 储存下一个月的月份
			let mut mon = 0;

			for weekday in 0..7 {
				// 这一天在一年中的序号
				// 若这一年不是从周一开始, 前面不存在的日子用负数表示
				let day = week * 7 + weekday - fst_weekday as i32;

				// 这一天是否是下一个月的第一天
				let is_next_mon =
					ord_mon_val != None && ord_mon_val.unwrap().1.clone() as i32 == day;
				is_next_mon_w = is_next_mon_w || is_next_mon;

				// 若日序号 == 十二月第一天的序号 + 十二月的天数
				// 即这一年已结束
				// 退出
				if day == (ord_mon[12 - 1] + 31) as i32 {
					write!(
					       f,
					       "*{}|\n",
					       format!(" {}", levels(u32::MAX)).repeat((7 - weekday) as usize)
					)?;
					break 'week;
				}
				let d_val = data_val.unwrap_or((&u32::MAX, &0));

				// 已经到了下一个月
				// 做个标记
				if is_next_mon {
					write!(f, "*")?;
					mon = ord_mon_val.unwrap().0 + 1;
					ord_mon_val = ord_mon_iter.next();
				} else {
					write!(f, " ")?;
				}

				// 这一天有数据
				// 打印出来
				if d_val.0.clone() as i32 == day {
					write!(f, "{}", levels(*d_val.1))?;
					data_val = data_iter.next();
				} else if day < 0 {
					write!(f, "{}", levels(u32::MAX))?;
				} else {
					write!(f, "{}", levels(0))?;
				}
			}
			// 打印月份
			if is_next_mon_w {
				write!(f, " | {:<2}\n", mon)?;
			} else {
				write!(f, " |\n")?;
			}
		}
		return Ok(());
	}
}

impl YearData {
	/// 打印到实现了 [`fmt::Write`] 特征的对象
	pub fn show_to_fmt(&self, year: i32, out: &mut impl fmt::Write) -> Result<()> {
		write!(out, "{}", self.data.get(&year).ok_or(Error::NoData)?)?;
		return Ok(());
	}

	/// 打印到实现了 [`io::Write`] 特征的对象
	pub fn show_to_io(&self, year: i32, out: &mut impl io::Write) -> Result<()> {
		write!(out, "{}", self.data.get(&year).ok_or(Error::NoData)?)?;
		return Ok(());
	}

	/// 打印到标准输出
	pub fn show(&self, year: i32) -> Result<()> {
		let mut stdout = io::stdout();
		self.show_to_io(year, &mut stdout)?;
		return Ok(());
	}
}
