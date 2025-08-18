use ratatui::{
	Frame,
	layout::{Constraint, Direction, Layout, Rect},
};

use crate::app::{App, CurrencyBlockType};

use super::{CURRENCY_BLOCK_WIDTH, currency::render_currency_block};

impl App {
	pub fn render_gain_blocks(&self, frame: &mut Frame, area: Rect) {
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
			"gain 1",
			&self.amount_input.gain1,
			self.active_currency_block == CurrencyBlockType::Gain1,
		);
		render_currency_block(
			frame,
			chunks[3],
			"gain 2",
			&self.amount_input.gain2,
			self.active_currency_block == CurrencyBlockType::Gain2,
		);
	}
}
