use rand::random;

use super::{Controller, ControllerError, Move};
use crate::game::Game;

#[derive(Debug, Default)]
pub struct RandomController;

impl Controller for RandomController {
	fn next_move(&mut self, _game: &Game) -> Result<Move, ControllerError> {
		let movement = random();
		Ok(movement)
	}
}
