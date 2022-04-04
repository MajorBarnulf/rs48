use termion::{event::Key, input::TermRead, raw::IntoRawMode};

use super::grid::Grid;
use std::{
	error::Error,
	fmt::Display,
	io::{stdin, stdout},
};

pub enum Move {
	LEFT,
	RIGHT,
	UP,
	DOWN,
}

#[derive(Debug)]
pub enum ControllerError {
	ExitSignal,
}

impl Display for ControllerError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let msg = match self {
			ControllerError::ExitSignal => "received exit signal",
		};
		f.write_str(msg)
	}
}

impl Error for ControllerError {}

pub trait Controller {
	fn next_move(&mut self, grid: &Grid) -> Result<Move, ControllerError>;
}

pub struct PlayerController {
	//
}

impl PlayerController {
	pub fn new() -> Self {
		Self {}
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
