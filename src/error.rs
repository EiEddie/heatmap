use std::io;

use crossterm::style::Stylize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	/// 没有数据源
	#[error(
	        "Have no source of data,
		use '-s' or set an environment variable named 'DATA_PATH'"
	)]
	NoSourceOfData,

	/// 给定的日期不合法
	#[error("Given date is wrong")]
	WrongDate,

	/// 查找的数据不存在
	#[error("Have no data about input")]
	NoData,

	#[error(transparent)]
	DataBaseError(#[from] ::rusqlite::Error),

	#[error(transparent)]
	ParseIntError(#[from] ::std::num::ParseIntError),

	#[error(transparent)]
	FmtError(#[from] ::std::fmt::Error),

	#[error(transparent)]
	IoError(#[from] ::std::io::Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;

pub fn error_handler(err: &Error, out: &mut impl io::Write) {
	if let Error::IoError(io_err) = err {
		if io_err.kind() == io::ErrorKind::BrokenPipe {
			::std::process::exit(0);
		}
	}

	writeln!(out, "{}: {}", "[error]".red(), err).ok();
}
