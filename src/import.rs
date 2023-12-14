use chrono::NaiveDate;
use rusqlite as rsql;

use crate::error::{Error, Result};
use crate::stat::YearData;

pub struct Database {
	conn: rsql::Connection,
}

impl Database {
	pub fn new(dbpath: &String) -> Result<Self> {
		return Ok(Self { conn: rsql::Connection::open(dbpath)?, });
	}

	/// 获取数据库中记录的所有年份
	pub fn get_all_year(&self) -> Result<Vec<i32>> {
		let mut all_year: Vec<i32> = Vec::new();

		for year in self.conn
		                .prepare("SELECT name FROM sqlite_master WHERE type='table'")?
		                .query_map([], |row| Ok(row.get::<usize, String>(0)?))?
		{
			all_year.push(year?.parse()?);
		}

		return Ok(all_year);
	}

	/// 将数据库中的所有记录导入 [`YearData`] 内
	pub fn data_into(&self, dest: &mut YearData) -> Result<()> {
		// todo!()
		for year in self.get_all_year()? {
			for i in self.conn
			             .prepare(format!("SELECT month, day FROM '{}'", year).as_str())?
			             .query_map([], |row| {
				             Ok((row.get::<usize, u32>(0)?, row.get::<usize, u32>(1)?))
			             })?
			{
				let i = i?;
				if let Some(date) = NaiveDate::from_ymd_opt(year, i.0, i.1) {
					dest.add(date);
				} else {
					return Err(Error::WrongDate);
				}
			}
		}
		return Ok(());
	}
}
