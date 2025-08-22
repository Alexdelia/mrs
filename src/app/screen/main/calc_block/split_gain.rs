use ratatui::{
	Frame,
	layout::{Constraint, Direction, Layout, Rect},
	style::Color,
};

use crate::app::App;

use super::{CURRENCY_BLOCK_WIDTH, currency::render_currency_block};

impl App {
	pub fn render_split_gain_blocks(&self, frame: &mut Frame, area: Rect) {
		let chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([
				Constraint::Min(0),
				Constraint::Max(CURRENCY_BLOCK_WIDTH),
				Constraint::Length(1),
				Constraint::Max(CURRENCY_BLOCK_WIDTH),
				Constraint::Min(0),
			])
			.split(area);

		render_currency_block(
			frame,
			chunks[1],
			"split 1",
			&format!("{:.2}", self.future_row.split_gain1()),
			Color::Magenta,
		);
		render_currency_block(
			frame,
			chunks[3],
			"split 2",
			&format!("{:.2}", self.future_row.split_gain2()),
			Color::Magenta,
		);
	}
}
