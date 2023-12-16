use chrono::NaiveDate;

use crate::error::Result;
use crate::stat::YearData;
pub mod sqlite;

pub trait Import {
	/// 获取所有数据
	fn all_datas(self) -> Result<Vec<NaiveDate>>;
}

impl YearData {
	/// 从别处导入数据创建
	pub fn from(other: impl Import) -> Result<Self> {
		let mut me = Self::new();
		for date in other.all_datas()? {
			me.add(date);
		}
		return Ok(me);
	}
}
