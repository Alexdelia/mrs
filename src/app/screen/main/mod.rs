mod calc_block;
mod history_table;

use ratatui::{
	Frame,
	layout::{Constraint, Direction, Layout},
};

use crate::app::App;

impl App {
	pub fn render_main(&self, frame: &mut Frame) {
		let chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Fill(1), Constraint::Length(74)])
			.split(frame.area());

		self.render_calc_block(frame, chunks[0]);
		self.render_history_table(frame, chunks[1]);
	}
}
