use color_eyre::Result;
use ratatui::DefaultTerminal;

use super::App;

impl App {
	pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
		while !self.exit {
			terminal.draw(|frame| self.draw(frame))?;
			self.handle_event()?;
		}
		Ok(())
	}
}
