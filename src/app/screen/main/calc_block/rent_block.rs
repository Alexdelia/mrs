use ratatui::{
	Frame,
	layout::{Constraint, Direction, Layout, Rect},
};

use crate::app::{App, CurrencyBlockType};

use super::{CURRENCY_BLOCK_WIDTH, currency_block::render_currency_block};

impl App {
	pub fn render_rent_block(&self, frame: &mut Frame, area: Rect) {
		let chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([
				Constraint::Min(0),
				Constraint::Max(CURRENCY_BLOCK_WIDTH),
				Constraint::Min(0),
			])
			.split(area);

		render_currency_block(
			frame,
			chunks[1],
			"rent",
			&self.amount_input.rent,
			self.active_currency_block == CurrencyBlockType::Rent,
		);
	}
}
