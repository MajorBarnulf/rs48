use crate::grid::Grid;

use super::{Controller, ControllerError, Move};

pub enum Objective {
	Score,
	TileCount,
}

pub struct SimulatedController {
	_simulations_per_move: usize,
	_length_of_simulation: usize,
	_objective: Objective,
}

impl SimulatedController {
	pub fn new(
		_simulations_per_move: usize,
		_length_of_simulation: usize,
		_objective: Objective,
	) -> Self {
		Self {
			_simulations_per_move,
			_length_of_simulation,
			_objective,
		}
	}
}

impl Controller for SimulatedController {
	fn next_move(&mut self, _grid: &Grid) -> Result<Move, ControllerError> {
		todo!()
	}
}
