use ratatui::{
	Frame,
	layout::{Constraint, Direction, Layout, Rect},
	style::Color,
};

use crate::{Float, app::App};

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
			&format_percentage(self.future_row.percentage()),
			Color::Green,
		);
	}
}

fn format_percentage(amount: Float) -> String {
	format!("{:.8}", amount)
		.trim_end_matches('0')
		.trim_end_matches('.')
		.to_string()
}
