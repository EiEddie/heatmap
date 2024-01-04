use std::{env, io};

use heatmap::*;

fn run() -> error::Result<()> {
	let data_path = env::var("DATA_PATH").map_err(|_| error::Error::NoSourceOfData)?;

	let db = import::sqlite::Database::new(&data_path)?;

	stat::YearData::from(db)?.print_full()?;

	Ok(())
}

fn main() {
	if let Err(err) = run() {
		let mut stderr = io::stderr();
		error::error_handler(&err, &mut stderr);
	}
}
