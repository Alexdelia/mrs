mod path;

use color_eyre::Result;
use serde::Deserialize;

pub struct Data {
	rows: Vec<DataRow>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataRow {
	pub year: u16,
	pub month: u8,
	pub rent: f128,
	pub gain_1: f128,
	pub gain_2: f128,
}

impl Data {
	pub fn read() -> Result<Self> {
		let mut rows = Vec::new();

		let path = Self::data_path()?;

		if !path.exists() {
			return Ok(Self { rows });
		};

		let file = std::fs::File::open(path)?;
		let mut rdr = csv::ReaderBuilder::new()
			.has_headers(true)
			.from_reader(file);
		for result in rdr.deserialize() {
			let row: DataRow = result?;
			rows.push(row);
		}

		Ok(Self { rows })
	}
}
