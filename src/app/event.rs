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
			event::KeyCode::Up => {
				if self.active_currency_block != super::CurrencyBlockType::Rent {
					self.active_currency_block = super::CurrencyBlockType::Rent;
				}
			}
			event::KeyCode::Down => {
				if self.active_currency_block == super::CurrencyBlockType::Rent {
					self.active_currency_block = super::CurrencyBlockType::Gain2;
				}
			}
			event::KeyCode::Left => {
				if self.active_currency_block != super::CurrencyBlockType::Gain1 {
					self.active_currency_block = super::CurrencyBlockType::Gain1;
				}
			}
			event::KeyCode::Right => {
				if self.active_currency_block != super::CurrencyBlockType::Gain2 {
					self.active_currency_block = super::CurrencyBlockType::Gain2;
				}
			}
			_ => {}
		}
	}
}
