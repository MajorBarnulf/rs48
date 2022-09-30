use std::ops::Add;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{game::Game, prelude::RandomController};

use super::{Controller, ControllerError, Move};

pub struct SimulatedController {
	simulations_per_move: usize,
	length_of_simulation: usize,
}

impl SimulatedController {
	pub fn new(simulations_per_move: usize, length_of_simulation: usize) -> Self {
		Self {
			simulations_per_move,
			length_of_simulation,
		}
	}
}

impl Controller for SimulatedController {
	fn next_move(&mut self, game: &Game) -> Result<Move, ControllerError> {
		let initial_score = game.get_score();

		let mut scores: Vec<_> = Move::all()
			.into_par_iter()
			.map(|initial_move| {
				let sim_scores = (0..self.simulations_per_move).into_par_iter().map(|_| {
					let mut game = game.clone();

					game.turn(initial_move.clone()).ok();
					let mut controller = RandomController::default();
					for _ in 1..self.length_of_simulation {
						let movement = controller.next_move(&game).ok();
						let result = movement.and_then(|movement| game.turn(movement).ok());
						if result.is_none() {
							break;
						}
					}
					game.get_score() - initial_score
				});

				let sim_scores: Vec<_> = sim_scores.collect();
				let avg =
					sim_scores.iter().cloned().reduce(Add::add).unwrap_or(0) / sim_scores.len();
				(initial_move, avg)
			})
			.collect();
		scores.sort_by(|(_, a), (_, b)| b.cmp(a));

		let (m, _) = scores.first().unwrap();
		Ok(m.clone())
	}
}
