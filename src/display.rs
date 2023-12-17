use std::fmt::Display;

use chrono::{Datelike, NaiveDate};

use crate::stat::DaysData;

impl Display for DaysData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// 每一个月第一天在这一年的序数
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
				// 负数被溢出到最大值附近
				let day = (week * 7 + weekday - fst_weekday as i32) as u32;

				// 这一天是否是下一个月的第一天
				let is_next_mon = ord_mon_val != None && ord_mon_val.unwrap().1.clone() == day;
				is_next_mon_w = is_next_mon_w || is_next_mon;

				// 储存的数据已显示完毕
				// 退出
				if data_val == None {
					write!(f, "{} |\n", "  ".repeat((7 - weekday) as usize))?;
					break 'week;
				}
				let d_val = data_val.unwrap();

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
				if d_val.0.clone() == day {
					// TODO: 打印热力图而非数字
					write!(f, "{}", d_val.1)?;
					data_val = data_iter.next();
				} else {
					write!(f, " ")?;
				}
			}
			// TODO: 彩色输出
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
