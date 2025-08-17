use ratatui::{Frame, buffer::Buffer, layout::Rect, text::Text, widgets::Widget};

use crate::app::App;

impl App {
	pub fn render_graph(&self, frame: &mut Frame) {
		frame.render_widget(Text::from("Graph View"), frame.area());
	}
}
