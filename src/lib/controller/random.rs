use rand::random;

use super::{Controller, ControllerError, Move};
use crate::lib::grid::Grid;

pub struct RandomController;

impl RandomController {
	pub fn new() -> Self {
		Self
	}
}

impl Controller for RandomController {
	fn next_move(&mut self, _grid: &Grid) -> Result<Move, ControllerError> {
		let movement = random();
		Ok(movement)
	}
}
