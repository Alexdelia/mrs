mod app;
mod history;

use app::App;
use color_eyre::Result;
use history::History;

type Float = f64;

fn main() -> Result<()> {
	color_eyre::install()?;

	let history = History::read()?;

	let mut terminal = ratatui::init();
	let result = App::new(history).run(&mut terminal);

	ratatui::restore();

	result
}
