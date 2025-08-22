use ratatui::{
	Frame,
	layout::{Constraint, Direction, Layout, Rect},
	style::Color,
};

use crate::app::App;

use super::amount::render_amount_block;

impl App {
	pub fn render_percentage_block(&self, frame: &mut Frame, area: Rect) {
		let chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Min(0), Constraint::Max(21), Constraint::Min(0)])
			.split(area);

		render_amount_block(
			frame,
			chunks[1],
			"percentage",
			"%",
			&(format!("{:.8}", &self.future_row.percentage())
				.trim_end_matches('0')
				.trim_end_matches('.')
				.to_string()),
			Color::Green,
		);
	}
}
