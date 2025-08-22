use ratatui::{
	Frame,
	layout::{Alignment, Rect},
	style::{Color, Modifier, Style},
	text::Line,
	widgets::{Block, BorderType, Borders, Padding},
};

pub fn render_amount_block(
	frame: &mut Frame,
	area: Rect,
	title: &str,
	unit: &str,
	amount: &str,
	border_color: Color,
) {
	let block = Block::default()
		.title(Line::from(title).left_aligned())
		.title(Line::from(unit).right_aligned())
		.borders(Borders::ALL)
		.border_type(BorderType::Rounded)
		.style(
			Style::default()
				.fg(border_color)
				.remove_modifier(Modifier::BOLD),
		)
		.padding(Padding::new(0, 0, 1, 1));

	let paragraph = ratatui::widgets::Paragraph::new(amount)
		.block(block)
		.style(
			Style::default()
				.fg(Color::White)
				.add_modifier(Modifier::BOLD),
		)
		.alignment(Alignment::Center);

	frame.render_widget(paragraph, area);
}
