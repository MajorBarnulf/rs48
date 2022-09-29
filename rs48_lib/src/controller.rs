use rand::{distributions::Standard, prelude::Distribution};

use crate::game::Game;
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum Move {
	LEFT,
	RIGHT,
	UP,
	DOWN,
}

impl Move {
	pub fn all() -> [Self; 4] {
		[Self::LEFT, Self::RIGHT, Self::UP, Self::DOWN]
	}
}

impl Distribution<Move> for Standard {
	fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Move {
		match rng.gen_range(0..4) {
			0 => Move::DOWN,
			1 => Move::LEFT,
			2 => Move::RIGHT,
			_ => Move::UP,
		}
	}
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
	fn next_move(&mut self, game: &Game) -> Result<Move, ControllerError>;

	fn into_box(self) -> Box<dyn Controller>
	where
		Self: Sized + 'static,
	{
		Box::new(self)
	}
}

pub mod player;
pub mod random;
pub mod simulated;

pub use player::PlayerController;
pub use random::RandomController;
pub use simulated::SimulatedController;
