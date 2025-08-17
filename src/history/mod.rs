mod io;
mod path;

use serde::{Deserialize, Serialize};

use crate::Float;

pub struct History {
	pub rows: Vec<HistoryRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRow {
	pub year: u16,
	pub month: u8,
	pub rent: Float,
	pub gain_1: Float,
	pub gain_2: Float,
}
