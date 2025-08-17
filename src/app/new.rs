use crate::history::{History, HistoryRow};

use super::{App, Tab};

impl App {
	pub fn new(history: History) -> Self {
		Self {
			tab: Tab::Main,
			future_row: history
				.rows
				.first()
				.map_or_else(|| HistoryRow::default(), |row| row.clone()),
			history,
			exit: false,
		}
	}
}
