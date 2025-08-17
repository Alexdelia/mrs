mod rent_block;

use ratatui::{
	Frame,
	layout::{Constraint, Direction, Layout, Rect},
	style::{Color, Style},
	text::Text,
	widgets::{Block, BorderType, Borders},
};

use crate::app::App;

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
	}
}
