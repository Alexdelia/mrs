use ratatui::{
	Frame,
	layout::{Constraint, Direction, Layout, Rect},
	style::{Color, Style},
	widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::app::App;

impl App {
	pub fn render_rent_block(&self, frame: &mut Frame, area: Rect) {
		let chunks = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Min(0), Constraint::Max(14), Constraint::Min(0)])
			.split(area);

		let rent_block = Block::default()
			.borders(Borders::ALL)
			.border_type(BorderType::Rounded)
			.title("rent")
			.style(Style::default().fg(Color::White))
			.padding(Padding::new(0, 0, 1, 1));

		let rent_paragraph = Paragraph::new(format!("{:.2}€", self.future_row.rent))
			.style(Style::default().fg(Color::White))
			.alignment(ratatui::layout::Alignment::Center)
			.block(rent_block);

		frame.render_widget(rent_paragraph, chunks[1]);
	}
}
