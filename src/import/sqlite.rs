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
}

impl Import for Database {
	fn all_datas(self) -> Result<Vec<(NaiveDate, u32)>> {
		let mut all_datas: Vec<(NaiveDate, u32)> = Vec::new();

		for date_time in
			self.conn
			    .prepare("SELECT year, month, day, COALESCE(time, '07: 00') FROM main")?
			    .query_map([], |row| {
				    Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get::<_, String>(3)?))
			    })? {
			let (year, month, day, time) = date_time?;
			let time = NaiveTime::parse_from_str(&time, "%H: %M").map_err(|_| Error::WrongDate)?;

			// 是否是前一天, 若是, 此值为 1 天
			// 早晨七点之前将判定为前一天
			let prev_day_offset = Days::new(if time < NaiveTime::from_hms_opt(7, 0, 0).unwrap() {
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

		Ok(all_datas)
	}
}
