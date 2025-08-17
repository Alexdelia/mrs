use color_eyre::Result;
use std::path::PathBuf;

use super::Data;

const DATA_FILE: &str = "data.csv";

impl Data {
	pub fn data_path() -> Result<PathBuf> {
		let mut dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
		dir.push(env!("CARGO_CRATE_NAME"));
		if !dir.exists() {
			std::fs::create_dir_all(&dir)?;
		}

		dir.push(DATA_FILE);
		Ok(dir)
	}
}
