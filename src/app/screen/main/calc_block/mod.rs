pub mod currency_block;
mod gain_blocks;
mod rent_block;

use ratatui::{
	Frame,
	layout::{Constraint, Direction, Layout, Rect},
};

use crate::app::App;

const CURRENCY_BLOCK_WIDTH: u16 = 13;

impl App {
	pub fn render_calc_block(&self, frame: &mut Frame, area: Rect) {
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([
				Constraint::Min(0),
				Constraint::Max(5),
				Constraint::Max(5),
				Constraint::Min(0),
			])
			.split(area);

		self.render_rent_block(frame, chunks[1]);
		self.render_gain_blocks(frame, chunks[2]);
	}
}
