use std::{error::Error, fmt::Display};

use super::{
	controller::{ControllerError, Move},
	grid::Grid,
};

pub struct Rules {
	size: usize,
	spawn_per_turn: usize,
}

impl Rules {
	pub fn size(mut self, size: usize) -> Self {
		self.size = size;
		self
	}

	pub fn spawn_per_turn(mut self, spawn_per_turn: usize) -> Self {
		self.spawn_per_turn = spawn_per_turn;
		self
	}
}

impl Default for Rules {
	fn default() -> Self {
		Self {
			size: 4,
			spawn_per_turn: 1,
		}
	}
}

#[derive(Debug)]
pub enum GameError {
	GridIsFull,
	ControllerError(ControllerError),
}

impl From<ControllerError> for GameError {
	fn from(error: ControllerError) -> Self {
		Self::ControllerError(error)
	}
}

impl Display for GameError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::GridIsFull => f.write_str("grid is full"),
			GameError::ControllerError(err) => err.fmt(f),
		}
	}
}

impl Error for GameError {}

#[derive(Clone)]
pub struct Game {
	board: Grid,
	score: usize,
	turn_index: usize,
	spawn_per_turn: usize,
}

impl Game {
	pub fn new(rules: Rules) -> Self {
		let Rules {
			size,
			spawn_per_turn,
		} = rules;

		Self {
			board: Grid::new(size),
			score: 0,
			turn_index: 0,
			spawn_per_turn,
		}
	}

	pub fn get_board(&self) -> &Grid {
		&self.board
	}

	pub fn get_score(&self) -> usize {
		self.score
	}

	pub fn get_turn_index(&self) -> usize {
		self.turn_index
	}

	pub fn turn(&mut self, movement: Move) -> Result<(), GameError> {
		let move_score = self.perform_move(movement);
		self.score += move_score;
		for _ in 0..self.spawn_per_turn {
			self.spawn_random()?;
		}
		self.turn_index += 1;
		Ok(())
	}

	fn spawn_random(&mut self) -> Result<(), GameError> {
		let mut potentials = vec![];
		for x in 0..self.board.size() {
			for y in 0..self.board.size() {
				if self
					.board
					.get((x, y))
					.expect("coordinates are valid")
					.is_empty()
				{
					potentials.push((x, y));
				}
			}
		}
		let potential_count = potentials.len() as f32;
		if potential_count == 0. {
			return Err(GameError::GridIsFull);
		}
		let random = rand::random::<f32>() * potential_count;
		let index = random.floor() as usize;
		let (x, y) = potentials[index];
		self.board.set((x, y), Some(1));
		Ok(())
	}

	pub fn perform_move(&mut self, movement: Move) -> usize {
		let mut move_score = 0;
		match movement {
			Move::LEFT => {
				for y in 0..self.board.size() {
					for x in 0..self.board.size() {
						move_score += self.perform_linear_move((-1, 0), (x, y));
					}
				}
			}
			Move::RIGHT => {
				for y in 0..self.board.size() {
					for x in (0..self.board.size()).rev() {
						move_score += self.perform_linear_move((1, 0), (x, y));
					}
				}
			}
			Move::UP => {
				for x in 0..self.board.size() {
					for y in 0..self.board.size() {
						move_score += self.perform_linear_move((0, -1), (x, y));
					}
				}
			}
			Move::DOWN => {
				for x in 0..self.board.size() {
					for y in (0..self.board.size()).rev() {
						move_score += self.perform_linear_move((0, 1), (x, y));
					}
				}
			}
		};
		move_score
	}

	fn perform_linear_move(
		&mut self,
		direction: (isize, isize),
		tile_pos: (usize, usize),
	) -> usize {
		if self
			.board
			.get(tile_pos)
			.expect("function should only be called internally with known coordinates")
			.is_empty()
		{
			0
		} else {
			let mut displacement = Displacement::new(&mut self.board, tile_pos, direction);
			displacement.move_all();
			displacement.pop_score()
		}
	}
}

pub struct Displacement<'g> {
	grid: &'g mut Grid,
	position: (usize, usize),
	direction: (isize, isize),
	score: usize,
}

impl<'g> Displacement<'g> {
	pub fn new(grid: &'g mut Grid, position: (usize, usize), direction: (isize, isize)) -> Self {
		Self {
			grid,
			position,
			direction,
			score: 0,
		}
	}

	pub fn pop_score(self) -> usize {
		let Displacement { score, .. } = self;
		score
	}

	pub fn move_all(&mut self) {
		loop {
			let can_continue = self.move_once();
			if !can_continue {
				break;
			}
		}
	}

	fn move_once(&mut self) -> bool {
		let current_pos = self.position;
		let current_value = self
			.grid
			.get_val(current_pos)
			.expect("last position should be valid");
		if let Some(next_pos) = self.get_next_pos() {
			match self.grid.get_val(next_pos) {
				None => {
					self.grid.move_tile(current_pos, next_pos);
					self.set_pos(next_pos);
					true
				}
				Some(value) if value == current_value => {
					self.grid.move_tile(current_pos, next_pos);
					self.grid.set(next_pos, Some(value * 2));
					self.score = value * 2;
					false
				}
				Some(_) => false,
			}
		} else {
			false
		}
	}

	fn get_next_pos(&self) -> Option<(usize, usize)> {
		let (current_x, current_y) = self.position;
		let (dx, dy) = self.direction;
		if would_overflow(current_x, dx, self.grid.size() - 1)
			|| would_overflow(current_y, dy, self.grid.size() - 1)
		{
			None
		} else {
			let next_x = (current_x as isize) + dx;
			let next_y = (current_y as isize) + dy;
			Some((next_x as usize, next_y as usize))
		}
	}

	fn set_pos(&mut self, (x, y): (usize, usize)) {
		self.position = (x, y);
	}
}

/// determine if the given number, added a delta that is either 1 or -1 to it, would overflow a certain maximum value for n
fn would_overflow(number: usize, delta: isize, max: usize) -> bool {
	let too_little = number == 0 && delta == -1;
	let too_big = number == max && delta == 1;
	too_little || too_big
}
