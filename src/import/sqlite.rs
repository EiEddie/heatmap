use chrono::NaiveDate;
use rusqlite as rsql;

use super::Import;
use crate::error::{Error, Result};

pub struct Database {
	conn: rsql::Connection,
}

impl Database {
	pub fn new(dbpath: &str) -> Result<Self> {
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
}

impl Import for Database {
	fn all_datas(self) -> Result<Vec<(NaiveDate, u32)>> {
		let mut all_datas: Vec<(NaiveDate, u32)> = Vec::new();

		for year in self.get_all_year()? {
			for i in self.conn
			             .prepare(format!("SELECT month, day FROM '{}'", year).as_str())?
			             .query_map([], |row| {
				             Ok((row.get::<usize, u32>(0)?, row.get::<usize, u32>(1)?))
			             })?
			{
				let i = i?;
				if let Some(date) = NaiveDate::from_ymd_opt(year, i.0, i.1) {
					// 每个日期可能出现多次, 但是可以保证每次只记一次数
					all_datas.push((date, 1));
				} else {
					return Err(Error::WrongDate);
				}
			}
		}

		return Ok(all_datas);
	}
}
