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
	pub gain1: Float,
	pub gain2: Float,
}

impl HistoryRow {
	#[inline(always)]
	fn percentage(value: Float, total: Float) -> Float {
		if total == 0.0 {
			0.0
		} else {
			(value / total) * 100.0
		}
	}

	pub fn gain1_percentage(&self) -> Float {
		return Self::percentage(self.gain1, self.rent);
	}

	pub fn gain2_percentage(&self) -> Float {
		Self::percentage(self.gain2, self.rent)
	}
}
