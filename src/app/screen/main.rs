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
			.constraints([Constraint::Fill(1), Constraint::Length(20)])
			.split(frame.area());

		frame.render_widget(Text::from("calc"), chunks[0]);
		frame.render_widget(Text::from("history"), chunks[1]);
	}
}
