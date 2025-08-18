pub mod currency;
mod gain;
mod month;
mod rent;

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
				Constraint::Length(3),
				Constraint::Length(1),
				Constraint::Max(5),
				Constraint::Max(5),
				Constraint::Min(0),
			])
			.split(area);

		self.render_month_block(frame, chunks[1]);

		self.render_rent_block(frame, chunks[3]);
		self.render_gain_blocks(frame, chunks[4]);
	}
}
