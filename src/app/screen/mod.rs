use ratatui::{Frame, buffer::Buffer, layout::Rect, text::Text, widgets::Widget};

use super::App;

impl App {
	pub fn draw(&self, frame: &mut Frame) {
		frame.render_widget(self, frame.area());
	}
}

impl Widget for &App {
	fn render(self, area: Rect, buf: &mut Buffer) {
		Text::from("Hello, world!").render(area, buf);
	}
}
