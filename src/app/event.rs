use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyEventKind};

use super::{App, Tab};

impl App {
	pub fn handle_event(&mut self) -> Result<()> {
		match event::read()? {
			Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
				self.handle_key_event(key_event)?;
			}
			_ => {}
		};
		Ok(())
	}

	fn handle_key_event(&mut self, key_event: event::KeyEvent) -> Result<()> {
		match key_event.code {
			// quit
			event::KeyCode::Char('q') | event::KeyCode::Esc => {
				self.exit = true;
			}
			// tab switch
			event::KeyCode::Tab => {
				self.tab = match self.tab {
					Tab::Main => Tab::Graph,
					Tab::Graph => Tab::Main,
				};
			}
			// currency block selection
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
			// store future row
			event::KeyCode::Enter => {
				if let Some(last_row) = self.history.rows.last()
					&& last_row.year == self.future_row.year
						&& last_row.month == self.future_row.month
					{
						self.history.rows.pop();
					}
				self.history.rows.push(self.future_row.clone());
				self.history.write()?;
			}

			_ => self.handle_amount_input(key_event)?,
		}

		Ok(())
	}

	fn handle_amount_input(&mut self, key_event: event::KeyEvent) -> Result<()> {
		let (amount_str, amount_value) = match self.active_currency_block {
			super::CurrencyBlockType::Rent => {
				(&mut self.amount_input.rent, &mut self.future_row.rent)
			}
			super::CurrencyBlockType::Gain1 => {
				(&mut self.amount_input.gain1, &mut self.future_row.gain1)
			}
			super::CurrencyBlockType::Gain2 => {
				(&mut self.amount_input.gain2, &mut self.future_row.gain2)
			}
		};

		match key_event.code {
			event::KeyCode::Char(c) if c.is_ascii_digit() => {
				if amount_str == "0" {
					amount_str.clear();
				}
				amount_str.push(c);
				*amount_value = amount_str.parse::<f64>()?;
			}
			event::KeyCode::Char('.') => {
				if !amount_str.contains('.') {
					amount_str.push('.');
				}
			}
			event::KeyCode::Backspace => {
				if amount_str.pop().is_some() {
					*amount_value = amount_str.parse::<f64>().unwrap_or(0.0);
				}
			}
			event::KeyCode::Delete => {
				amount_str.clear();
				*amount_value = 0.0;
			}
			_ => {}
		}

		Ok(())
	}
}
