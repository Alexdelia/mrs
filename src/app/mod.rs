mod event;
mod new;
mod render;
mod run;

use crate::history::History;

pub struct App {
	pub tab: Tab,
	pub history: History,
	pub exit: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
	Main,
	Graph,
}
