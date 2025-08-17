use crate::history::{History, HistoryRow};

use super::{AmountInput, App, CurrencyBlockType, Tab};

impl App {
	pub fn new(history: History) -> Self {
		let future_row = history
			.rows
			.first()
			.map_or_else(|| HistoryRow::default(), |row| row.clone());

		let amount_input = AmountInput::from(&future_row);

		Self {
			tab: Tab::Main,
			active_currency_block: if future_row.gain2 != 0.0 {
				CurrencyBlockType::Gain2
			} else {
				CurrencyBlockType::Rent
			},
			amount_input,
			future_row,
			history,
			exit: false,
		}
	}
}

impl AmountInput {
	pub fn from(row: &HistoryRow) -> Self {
		Self {
			rent: row.rent.to_string(),
			gain1: row.gain1.to_string(),
			gain2: row.gain2.to_string(),
		}
	}
}
