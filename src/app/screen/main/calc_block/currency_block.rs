use ratatui::{
	Frame,
	layout::{Alignment, Rect},
	style::{Color, Modifier, Style},
	text::Line,
	widgets::{Block, BorderType, Borders, Padding},
};

pub fn render_currency_block(
	frame: &mut Frame,
	area: Rect,
	title: &str,
	amount: &str,
	is_active: bool,
) {
	let block = Block::default()
		.title(Line::from(title).left_aligned())
		.title(Line::from("€").right_aligned())
		.borders(Borders::ALL)
		.border_type(BorderType::Rounded)
		.style(
			Style::default()
				.fg(if is_active {
					Color::Cyan
				} else {
					Color::DarkGray
				})
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
