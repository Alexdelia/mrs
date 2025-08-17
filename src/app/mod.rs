mod event;
mod new;
mod run;
mod screen;

use crate::history::{History, HistoryRow};

pub struct App {
	pub tab: Tab,
	pub future_row: HistoryRow,
	pub history: History,
	pub exit: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
	Main,
	Graph,
}
