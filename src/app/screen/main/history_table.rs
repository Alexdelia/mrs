use ratatui::{
	Frame,
	layout::{Constraint, Rect},
	style::{Color, Style},
	widgets::{Block, Borders, Cell, Row, Table},
};

use crate::app::App;

impl App {
	pub fn render_history_table(&self, frame: &mut Frame, area: Rect) {
		let block = Block::default().borders(Borders::ALL).title("history");

		let header = [
			"Y", "M", "rent", "gain 1", "gain 2", "percent", "split 1", "split 2",
		]
		.into_iter()
		.map(Cell::from)
		.collect::<Row>()
		.style(Style::default().fg(Color::Gray))
		.height(1);

		let rows = self.history.rows.iter().map(|entry| {
			Row::new([
				entry.year.to_string(),
				format!("{:02}", entry.month),
				format!("{:.2}€", entry.rent),
				format!("{:.2}€", entry.gain1),
				format!("{:.2}€", entry.gain2),
				format!("{:.2}%", entry.percentage()),
				format!("{:.2}€", entry.split_gain1()),
				format!("{:.2}€", entry.split_gain2()),
			])
			.style(Style::default().fg(Color::White))
		});

		let currency_width = Constraint::Length(10);
		let percentage_width = Constraint::Length(9);
		let table = Table::new(
			rows,
			[
				Constraint::Length(4),
				Constraint::Length(2),
				currency_width,
				currency_width,
				currency_width,
				percentage_width,
				currency_width,
				currency_width,
			],
		)
		.header(header)
		.block(block);

		frame.render_widget(table, area);
	}
}
