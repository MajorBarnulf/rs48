use std::io::{stdin, stdout};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use super::{Controller, ControllerError, Move};
use crate::lib::grid::Grid;

pub struct PlayerController;

impl PlayerController {
	pub fn new() -> Self {
		Self
	}
}

impl Controller for PlayerController {
	fn next_move(&mut self, _grid: &Grid) -> Result<Move, ControllerError> {
		let stdin = stdin();
		let mut _stdout = stdout().into_raw_mode().unwrap();
		for c in stdin.keys() {
			let movement = match c.unwrap() {
				Key::Char('q') => return Err(ControllerError::ExitSignal),
				Key::Left => Move::LEFT,
				Key::Right => Move::RIGHT,
				Key::Up => Move::UP,
				Key::Down => Move::DOWN,
				_ => continue,
			};
			return Ok(movement);
		}
		unreachable!()
	}
}
