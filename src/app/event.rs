use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyEventKind};

use super::{App, Tab};

impl App {
	pub fn handle_event(&mut self) -> Result<()> {
		match event::read()? {
			Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
				self.handle_key_event(key_event)
			}
			_ => {}
		};
		Ok(())
	}

	fn handle_key_event(&mut self, key_event: event::KeyEvent) {
		match key_event.code {
			event::KeyCode::Char('q') | event::KeyCode::Esc => {
				self.exit = true;
			}
			event::KeyCode::Tab => {
				self.tab = match self.tab {
					Tab::Main => Tab::Graph,
					Tab::Graph => Tab::Main,
				};
			}
			_ => {}
		}
	}
}
