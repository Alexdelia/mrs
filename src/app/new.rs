use crate::history::History;

use super::{App, Tab};

impl App {
	pub fn new(history: History) -> Self {
		Self {
			tab: Tab::Main,
			history,
			exit: false,
		}
	}
}
