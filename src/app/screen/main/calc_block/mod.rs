mod amount;
pub mod currency;
mod gain;
mod month;
mod percentage;
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
				Constraint::Length(3), // year - month
				Constraint::Length(1), // gap
				Constraint::Max(5),    // rent
				Constraint::Max(5),    // gain 1 & gain 2
				Constraint::Length(1), // gap
				Constraint::Max(5),    // %
				Constraint::Max(5),    // split 1 & split 2
				Constraint::Min(0),
			])
			.split(area);

		self.render_month_block(frame, chunks[1]);

		self.render_rent_block(frame, chunks[3]);
		self.render_gain_blocks(frame, chunks[4]);

		self.render_percentage_block(frame, chunks[6]);
		// self.render_split_blocks(frame, chunks[7]);
	}
}
