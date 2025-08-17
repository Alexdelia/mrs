use color_eyre::Result;

use super::{History, HistoryRow};

impl History {
	pub fn read() -> Result<Self> {
		let mut rows = Vec::new();

		let path = Self::file_path()?;

		if !path.exists() {
			return Ok(Self { rows });
		};

		let file = std::fs::File::open(path)?;
		let mut rdr = csv::ReaderBuilder::new()
			.has_headers(true)
			.from_reader(file);
		for result in rdr.deserialize() {
			let row: HistoryRow = result?;
			rows.push(row);
		}

		Ok(Self { rows })
	}

	pub fn write(&mut self) -> Result<()> {
		let path = Self::file_path()?;
		let mut wtr = csv::WriterBuilder::new()
			.has_headers(true)
			.from_path(path)?;

		for row in &self.rows {
			wtr.serialize(row)?;
		}

		wtr.flush()?;
		Ok(())
	}
}
