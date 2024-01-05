use chrono::NaiveDate;

use crate::error::Result;
use crate::stat::YearData;
pub mod sqlite;

pub trait Import {
	/// 获取所有数据
	///
	/// # Return
	///
	/// 返回 `Reslut`, 包含一个 pair 的 vec,
	/// 这个 pair 记录着日期 [`NaiveDate`] 和该天的次数 [`u32`].
	///
	/// # Note
	///
	/// 日期可能重复出现; 多次出现时也可能带有不同的次数值.
	fn all_datas(self) -> Result<Vec<(NaiveDate, u32)>>;
}

impl YearData {
	/// 从别处导入数据创建
	pub fn from(other: impl Import) -> Result<Self> {
		let mut me = Self::new();
		for (date, times) in other.all_datas()? {
			me.add(date, times);
		}
		return Ok(me);
	}
}
