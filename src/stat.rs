use std::collections::BTreeMap;

use chrono::{Datelike, NaiveDate};

/// 每一天的数据
#[derive(Debug)]
pub(crate) struct DaysData {
	/// 储存的是哪一年的数据
	pub(crate) year: i32,

	/// 储存的值
	/// - `key`: 距一年第一天的日数, 从 0 开始
	/// - `val`: 这一天的统计数据
	pub(crate) data: BTreeMap<u32, u32>,
}

impl DaysData {
	fn new(year: i32) -> Self {
		return Self { year,
		              data: BTreeMap::new() };
	}

	/// 添加一条记录, 并返回给定日期的统计总次数
	///
	/// 若日期存在, 则给该日期的统计加指定的次数;
	/// 若不存在, 创建此日期的记录且加指定的次数.
	///
	/// # Warning
	///
	/// 当 `times` 为 0 时, 记录仍然会加 1.
	fn add(&mut self, date: NaiveDate, times: u32) -> u32 {
		// `times` 不得为 0, 因为添加一条空记录没有意义
		let times = if times == 0 { 1 } else { times };
		let cnt = self.data.entry(date.ordinal0()).or_insert(0);
		*cnt += times;
		return *cnt;
	}
}

/// 一年中每天的数据
///
/// 将每天的数据按年存放
#[derive(Debug)]
pub struct YearData {
	pub(crate) data: BTreeMap<i32, DaysData>,
}

impl YearData {
	pub fn new() -> Self {
		return Self { data: BTreeMap::new(), };
	}

	/// 添加一条记录, 并返回给定日期的统计总次数
	///
	/// 若日期存在, 则给该日期的统计加指定的次数;
	/// 若不存在, 创建此日期的记录且加指定的次数.
	///
	/// # Warning
	///
	/// 当 `times` 为 0 时, 记录仍然会加 1.
	pub fn add(&mut self, date: NaiveDate, times: u32) -> u32 {
		let year = date.year();
		let days_data = self.data.entry(year).or_insert(DaysData::new(year));
		return days_data.add(date, times);
	}
}
