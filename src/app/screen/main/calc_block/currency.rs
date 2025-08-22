use ratatui::{Frame, layout::Rect, style::Color};

use super::amount::render_amount_block;

pub fn render_currency_block(
	frame: &mut Frame,
	area: Rect,
	title: &str,
	amount: &str,
	is_active: bool,
) {
	render_amount_block(
		frame,
		area,
		title,
		"€",
		amount,
		if is_active {
			Color::Cyan
		} else {
			Color::DarkGray
		},
	);
}
