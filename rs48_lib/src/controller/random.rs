use rand::random;

use super::{Controller, ControllerError, Move};
use crate::game::Game;

pub struct RandomController;

impl RandomController {
	pub fn new() -> Self {
		Self
	}
}

impl Controller for RandomController {
	fn next_move(&mut self, _game: &Game) -> Result<Move, ControllerError> {
		let movement = random();
		Ok(movement)
	}
}
