use std::collections::BTreeMap;

use chrono::{Datelike, NaiveDate};

/// 每一天的数据
#[derive(Debug)]
struct DaysData {
	data: BTreeMap<NaiveDate, u32>,
	// TODO: 只包含月份和日期的结构代替 [`NaiveDate`]
	// 因为它与 [`YearData`] 中的信息重复
}

impl DaysData {
	fn new() -> Self {
		return Self { data: BTreeMap::new(), };
	}

	/// 添加一条记录, 并返回给定日期的统计总次数
	///
	/// 若日期存在, 则给该日期的统计 +1;
	/// 若不存在, 创建此日期的记录且 +1.
	fn add(&mut self, date: NaiveDate) -> u32 {
		let cnt = self.data.entry(date).or_insert(0);
		*cnt += 1;
		return *cnt;
	}

	fn get(&self, date: &NaiveDate) -> Option<u32> {
		return self.data.get(date).cloned();
	}
}

/// 一年中每天的数据
///
/// 将每天的数据按年存放
#[derive(Debug)]
pub struct YearData {
	data: BTreeMap<i32, DaysData>,
}

impl YearData {
	pub fn new() -> Self {
		return Self { data: BTreeMap::new(), };
	}

	/// 添加一条记录, 并返回给定日期的统计总次数
	///
	/// 若日期存在, 则给该日期的统计 +1;
	/// 若不存在, 创建此日期的记录且 +1.
	pub fn add(&mut self, date: NaiveDate) -> u32 {
		let days_data = self.data.entry(date.year()).or_insert(DaysData::new());
		return days_data.add(date);
	}
}
