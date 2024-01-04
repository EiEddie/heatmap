use std::fmt;

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

impl DaysData {
	/// 打印从 `from` 到 `to` 的全部统计数据到实现了 [`fmt::Write`] 的对象
	///
	/// `from` `to` 分别表示一年内某两天的序号, 从 0 开始
	fn fmt_range_year(&self, from: u32, to: u32, f: &mut impl fmt::Write) -> Result<()> {
		// `from` 应小于 `to`
		if to < from {
			return Err(Error::WrongDate);
		}

		let from_day = NaiveDate::from_yo_opt(self.year, from + 1).ok_or(Error::WrongDate)?;
		let to_day = NaiveDate::from_yo_opt(self.year, to + 1).ok_or(Error::WrongDate)?;

		// 给定的序列中每一个月第一天在这一年的序数
		// 从 0 计数
		let ord_mon: Vec<_> = (from_day.month()..=to_day.month()).map(|m| {
			                      NaiveDate::from_ymd_opt(self.year, m, 1).unwrap().ordinal0()
		                      })
		                      .collect();

		// 给定的序列中第一天的月份
		// 从 0 计数
		let from_mon = from_day.month0();

		// 这一年第一天是星期几
		// 从 0 计数
		let fst_weekday = NaiveDate::from_yo_opt(self.year, 1).unwrap()
		                                                      .weekday()
		                                                      .number_from_monday()
		                  - 1;

		let mut data_iter = self.data.iter().filter(|i| *i.0 >= from);
		let mut ord_mon_iter = ord_mon.iter().enumerate();

		let mut data_val = data_iter.next();
		let mut ord_mon_val = ord_mon_iter.next();

		// 这一周是否包含下一个月的第一天
		// 即两个月的交界处是否是这一周
		let mut is_next_mon_w = false;

		// 若这一周包含下一个月的第一天
		// 储存下一个月的月份
		let mut mon = 0;

		// 打印占位符, 使第一天所属的星期正确
		write!(
		       f,
		       "{}",
		       format!(" {}", levels(u32::MAX)).repeat(
			from_day.weekday().number_from_monday() as usize - 1
		)
		)?;

		for day in from..=to {
			// 修正后的日期序号
			// 若这一年不是从周一开始, 则调整为从周一开始的日期
			let day_fixed = day + fst_weekday + 1;

			// 这一天是否是下一个月的第一天
			let is_next_mon = ord_mon_val != None && ord_mon_val.unwrap().1.clone() == day;
			is_next_mon_w = is_next_mon_w || is_next_mon;

			let d_val = data_val.unwrap_or((&u32::MAX, &0));

			// 已经到了下一个月
			// 做个标记
			if is_next_mon {
				write!(f, "*")?;
				mon = ord_mon_val.unwrap().0 as u32 + from_mon + 1;
				ord_mon_val = ord_mon_iter.next();
			} else {
				write!(f, " ")?;
			}

			// 这一天有数据
			// 打印出来
			if d_val.0.clone() == day {
				write!(f, "{}", levels(*d_val.1))?;
				data_val = data_iter.next();
			} else {
				write!(f, "{}", levels(0))?;
			}

			// 这一周结束
			if day_fixed % 7 == 0 {
				// 打印月份
				if is_next_mon_w {
					write!(f, " | {:<2}\n", mon)?;
				} else {
					write!(f, " |\n")?;
				}
				is_next_mon_w = false;
			}
		}

		// 若给定范围末的一天不是周末
		// 打印末尾占位符
		let to_day_weekday = to_day.weekday().num_days_from_monday();
		if to_day_weekday != 6 {
			write!(
			       f,
			       "{} |\n",
			       format!(" {}", levels(u32::MAX)).repeat((6 - to_day_weekday) as usize)
			)?;
		}

		return Ok(());
	}
}

impl fmt::Display for DaysData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let end = NaiveDate::from_ymd_opt(self.year, 12, 31).unwrap()
		                                                    .ordinal0();
		return self.fmt_range_year(0, end, f).map_err(|_| fmt::Error);
	}
}

impl YearData {
	/// 打印一个日期范围的数据到实现了 [`fmt::Write`] 特征的对象
	// TODO: 打印指定的区间
	#[allow(dead_code)]
	fn fmt_range(&self, out: &mut impl fmt::Write, from: NaiveDate, to: NaiveDate) -> Result<()> {
		if from > to {
			return Err(Error::WrongDate);
		}

		for year in from.year()..=to.year() {
			let mut range = (u32::MAX, u32::MAX);

			range.0 = if year == from.year() {
				from.ordinal0()
			} else {
				0
			};

			range.1 = if year == to.year() {
				to.ordinal0()
			} else {
				(if NaiveDate::from_yo_opt(year, 1).unwrap().leap_year() {
					366
				} else {
					365
				}) - 1
			};

			write!(
			       out,
			       "{} |   {}\n",
			       format!(" {}", levels(u32::MAX)).repeat(7),
			       year
			)?;

			self.data
			    .get(&year)
			    .ok_or(Error::NoData)?
			    .fmt_range_year(range.0, range.1, out)?;
		}

		return Ok(());
	}

	/// 打印一年的数据到实现了 [`fmt::Write`] 特征的对象
	fn fmt_year(&self, out: &mut impl fmt::Write, year: i32) -> Result<()> {
		write!(out, "{}", self.data.get(&year).ok_or(Error::NoData)?)?;
		return Ok(());
	}

	/// 打印全部数据到实现了 [`fmt::Write`] 特征的对象
	fn fmt_all(&self, out: &mut impl fmt::Write) -> Result<()> {
		for (year, _) in &self.data {
			write!(
			       out,
			       "{} |   {}\n",
			       format!(" {}", levels(u32::MAX)).repeat(7),
			       year
			)?;
			self.fmt_year(out, *year)?;
		}
		return Ok(());
	}

	/// 打印一年的数据
	pub fn print_year(&self, year: i32) -> Result<()> {
		let mut buf = String::new();
		self.fmt_year(&mut buf, year)?;
		print!("{}", buf);
		return Ok(());
	}

	/// 打印全部数据
	pub fn print_full(&self) -> Result<()> {
		let mut buf = String::new();
		self.fmt_all(&mut buf)?;
		print!("{}", buf);
		return Ok(());
	}
}
