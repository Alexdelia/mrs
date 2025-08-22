mod io;
mod path;

use serde::{Deserialize, Serialize};

use crate::Float;

pub struct History {
	pub rows: Vec<HistoryRow>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct HistoryRow {
	pub year: u16,
	pub month: u8,
	pub rent: Float,
	pub gain1: Float,
	pub gain2: Float,
}

impl HistoryRow {
	#[inline]
	fn split_ratio(&self) -> Float {
		let total_gain = self.gain1 + self.gain2;
		if total_gain == 0.0 {
			return 0.0;
		}
		self.rent / total_gain
	}

	#[inline]
	pub fn percentage(&self) -> Float {
		self.split_ratio() * 100.0
	}

	#[inline]
	fn round_currency(value: Float) -> Float {
		(value * 100.0).round() / 100.0
	}

	#[inline]
	pub fn split_gain1(&self) -> Float {
		Self::round_currency(self.gain1 * self.split_ratio())
	}

	#[inline]
	pub fn split_gain2(&self) -> Float {
		Self::round_currency(self.gain2 * self.split_ratio())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_split_ratio() {
		let row = HistoryRow {
			year: 1970,
			month: 1,
			rent: 300.0,
			gain1: 400.0,
			gain2: 600.0,
		};
		assert_eq!(row.split_ratio(), 0.3);
		assert_eq!(row.percentage(), 30.0);
		assert_eq!(row.split_gain1(), 120.0);
		assert_eq!(row.split_gain2(), 180.0);
	}
}
