use color_eyre::Result;
use std::path::PathBuf;

use super::History;

const HISTORY_FILE_NAME: &str = "history.csv";

impl History {
	pub fn file_path() -> Result<PathBuf> {
		let mut dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
		dir.push(env!("CARGO_CRATE_NAME"));
		if !dir.exists() {
			std::fs::create_dir_all(&dir)?;
		}

		dir.push(HISTORY_FILE_NAME);
		Ok(dir)
	}
}
