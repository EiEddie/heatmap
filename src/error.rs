use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
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
