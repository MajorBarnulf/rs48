use super::grid::Grid;
use std::{error::Error, fmt::Display};

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

pub mod player;
