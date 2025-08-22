use chrono::{Datelike, Months};

use crate::history::{History, HistoryRow};

use super::{AmountInput, App, CurrencyBlockType, Tab};

impl App {
	pub fn new(history: History) -> Self {
		let mut future_row = history
			.rows
			.last()
			.map_or_else(HistoryRow::default, |row| row.clone());

		let now = chrono::Utc::now().date_naive();
		let last_month = now
			.checked_sub_months(Months::new(1))
			.expect("failed to subtract 1 month from now");
		future_row.year = last_month.year().try_into().expect("year out of range");
		future_row.month = last_month.month().try_into().expect("month out of range");

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
