use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	/// 给定的日期不合法
	#[error("Given date is wrong")]
	WrongDate,

	#[error(transparent)]
	DataBaseError(#[from] ::rusqlite::Error),

	#[error(transparent)]
	ParseIntError(#[from] ::std::num::ParseIntError),

	#[error(transparent)]
	FmtError(#[from] ::std::fmt::Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;
