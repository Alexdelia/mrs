mod event;
mod new;
mod run;
mod screen;

use ratatui::style::Color;

use crate::history::{History, HistoryRow};

const DEFAULT_BLOCK_BORDER_COLOR: Color = Color::DarkGray;

pub struct App {
	pub tab: Tab,
	pub active_currency_block: CurrencyBlockType,
	pub amount_input: AmountInput,
	pub future_row: HistoryRow,
	pub history: History,
	pub exit: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
	Main,
	Graph,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CurrencyBlockType {
	Rent,
	Gain1,
	Gain2,
}

#[derive(Default, Clone)]
pub struct AmountInput {
	pub rent: String,
	pub gain1: String,
	pub gain2: String,
}
