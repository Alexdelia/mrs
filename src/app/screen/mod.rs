mod graph;
mod main;

use ratatui::Frame;

use super::{App, Tab};

impl App {
	pub fn draw(&self, frame: &mut Frame) {
		match self.tab {
			Tab::Main => self.render_main(frame),
			Tab::Graph => self.render_graph(frame),
		}
	}
}
