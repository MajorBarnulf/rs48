use crate::lib::grid::Grid;

use super::{Controller, ControllerError, Move};

pub enum Objective {
	Score,
	TileCount,
}

pub struct SimulatedController {
	simulations_per_move: usize,
	length_of_simulation: usize,
	objective: Objective,
}

impl SimulatedController {
	pub fn new(
		simulations_per_move: usize,
		length_of_simulation: usize,
		objective: Objective,
	) -> Self {
		Self {
			simulations_per_move,
			length_of_simulation,
			objective,
		}
	}
}

impl Controller for SimulatedController {
	fn next_move(&mut self, grid: &Grid) -> Result<Move, ControllerError> {
		todo!()
	}
}
