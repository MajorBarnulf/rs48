use crate::{game::Game, prelude::RandomController};

use super::{Controller, ControllerError, Move};

pub enum Objective {
	Score,
	TileCount,
}

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
			.into_iter()
			.map(|initial_move| {
				let sim_scores = (0..self.simulations_per_move).map(|_| {
					let mut game = game.clone();
					game.turn(initial_move.clone()).ok();
					let mut controller = RandomController::new();
					for _ in 1..self.length_of_simulation {
						let movement = controller.next_move(&game).ok();
						let result = movement.and_then(|movement| game.turn(movement).ok());
						if result.is_none() {
							continue;
						}
					}
					game.get_score() - initial_score
				});

				let mut res = 0;
				let mut it = 0;
				for score in sim_scores {
					it += 1;
					res += score;
				}

				(initial_move, res / it)
			})
			.collect();
		scores.sort_by(|(_, a), (_, b)| a.cmp(b));

		let (m, _) = scores.last().unwrap();
		Ok(m.clone())
	}
}
