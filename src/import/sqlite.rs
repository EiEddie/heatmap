use chrono::{Days, NaiveDate, NaiveTime};
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
			             .prepare(format!(
				"SELECT month, day, COALESCE(time, '07: 00') FROM '{}'",
				year
			).as_str())?
			             .query_map([], |row| {
				             Ok((row.get(0)?, row.get(1)?, row.get::<_, String>(2).unwrap()))
			             })?
			{
				let (month, day, time) = i?;
				// dbg!(&time);
				let time =
					NaiveTime::parse_from_str(&time, "%H: %M").map_err(|_| Error::WrongDate)?;

				// 是否是前一天, 若是, 此值为 1 天
				// 早晨七点之前将判定为前一天
				let prev_day_offset =
					Days::new(if time < NaiveTime::from_hms_opt(7, 0, 0).unwrap() {
						1
					} else {
						0
					});

				if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
					// 每个日期可能出现多次, 但是可以保证每次只记一次数
					all_datas.push((date - prev_day_offset, 1));
				} else {
					return Err(Error::WrongDate);
				}
			}
		}

		return Ok(all_datas);
	}
}
