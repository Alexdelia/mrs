use ratatui::{Frame, layout::Rect, style::Color};

use crate::app::DEFAULT_BLOCK_BORDER_COLOR;

use super::amount::render_amount_block;

pub fn render_interactive_currency_block(
	frame: &mut Frame,
	area: Rect,
	title: &str,
	amount: &str,
	is_active: bool,
) {
	render_currency_block(
		frame,
		area,
		title,
		amount,
		if is_active {
			Color::Cyan
		} else {
			DEFAULT_BLOCK_BORDER_COLOR
		},
	);
}

pub fn render_currency_block(
	frame: &mut Frame,
	area: Rect,
	title: &str,
	amount: &str,
	color: Color,
) {
	render_amount_block(frame, area, title, "€", amount, color);
}
