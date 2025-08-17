mod history_table;

use ratatui::{
	Frame,
	layout::{Constraint, Direction, Layout},
	text::Text,
};

use crate::app::App;

impl App {
	pub fn render_main(&self, frame: &mut Frame) {
		let chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Fill(1), Constraint::Length(53)])
			.split(frame.area());

		frame.render_widget(Text::from("calc"), chunks[0]);
		self.render_history_table(frame, chunks[1]);
	}
}
