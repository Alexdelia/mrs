use crate::history::{History, HistoryRow};

use super::{App, CurrencyBlockType, Tab};

impl App {
	pub fn new(history: History) -> Self {
		let future_row = history
			.rows
			.first()
			.map_or_else(|| HistoryRow::default(), |row| row.clone());

		Self {
			tab: Tab::Main,
			active_currency_block: if future_row.gain2 != 0.0 {
				CurrencyBlockType::Gain2
			} else {
				CurrencyBlockType::Rent
			},
			future_row,
			history,
			exit: false,
		}
	}
}
