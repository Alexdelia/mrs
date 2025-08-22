use ratatui::{
	Frame,
	layout::{Constraint, Direction, Layout, Rect},
	style::Color,
};

use crate::app::{App, DEFAULT_BLOCK_BORDER_COLOR};

use super::{CURRENCY_BLOCK_WIDTH, amount::render_amount_block};

impl App {
	pub fn render_sum_validation_block(&self, frame: &mut Frame, area: Rect) {
		let chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([
				Constraint::Min(0),
				Constraint::Max(CURRENCY_BLOCK_WIDTH),
				Constraint::Min(0),
			])
			.split(area);

		let sum = self.future_row.split_gain1() + self.future_row.split_gain2();
		let delta = (sum - self.future_row.rent).abs();

		render_amount_block(
			frame,
			chunks[1],
			"sum valid",
			"€",
			&format!("{:.2}", sum),
			if sum == 0.0 {
				DEFAULT_BLOCK_BORDER_COLOR
			} else if delta > 0.01 {
				Color::Red
			} else if delta < 0.001 {
				Color::Green
			} else {
				Color::Yellow
			},
		);
	}
}
