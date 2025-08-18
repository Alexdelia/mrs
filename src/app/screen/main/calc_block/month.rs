use ratatui::{
	Frame,
	layout::{Alignment, Constraint, Direction, Layout, Rect},
	style::{Color, Style},
	widgets::{Block, BorderType, Borders},
};

use crate::app::App;

impl App {
	pub fn render_month_block(&self, frame: &mut Frame, area: Rect) {
		let chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([
				Constraint::Min(0),
				Constraint::Length(9),
				Constraint::Min(0),
			])
			.split(area);

		let block = Block::default()
			.title("period")
			.borders(Borders::ALL)
			.border_type(BorderType::Rounded)
			.style(Color::DarkGray);

		let paragraph = ratatui::widgets::Paragraph::new(format!(
			"{:04}-{:02}",
			self.future_row.year, self.future_row.month
		))
		.block(block)
		.style(Style::default().fg(Color::White))
		.alignment(Alignment::Center);

		frame.render_widget(paragraph, chunks[1]);
	}
}
