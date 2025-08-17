mod app;
mod history;

use color_eyre::Result;
use crossterm::event::{self, Event};
use history::History;
use ratatui::{DefaultTerminal, Frame};

type Float = f64;

fn main() -> Result<()> {
	color_eyre::install()?;

	let history = History::read()?;

	let terminal = ratatui::init();
	let result = run(terminal);

	ratatui::restore();

	result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
	loop {
		terminal.draw(render)?;
		if matches!(event::read()?, Event::Key(_)) {
			break Ok(());
		}
	}
}

fn render(frame: &mut Frame) {
	frame.render_widget("hello world", frame.area());
}
