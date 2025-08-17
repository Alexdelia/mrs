use ratatui::{
	Frame,
	layout::{Alignment, Rect},
	style::{Color, Style},
	text::Line,
	widgets::{Block, BorderType, Borders, Padding},
};

use crate::Float;

pub fn render_currency_block(
	frame: &mut Frame,
	area: Rect,
	title: &str,
	amount: Float,
	is_active: bool,
) {
	let block = Block::default()
		.title(Line::from(title).left_aligned())
		.title(Line::from("€").right_aligned())
		.borders(Borders::ALL)
		.border_type(BorderType::Rounded)
		.style(Style::default().fg(if is_active {
			Color::Cyan
		} else {
			Color::DarkGray
		}))
		.padding(Padding::new(0, 0, 1, 1));

	let paragraph = ratatui::widgets::Paragraph::new(amount.to_string())
		.block(block)
		.style(Style::default().fg(Color::White))
		.alignment(Alignment::Center);

	frame.render_widget(paragraph, area);
}
