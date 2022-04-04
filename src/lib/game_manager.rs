use std::{thread, time::Duration};

use crate::lib::{
	controller::Controller,
	game::{self, Game, GameError},
};

use super::grid_displayer::GridDisplayer;

pub struct Rules {
	display: bool,
	display_skips: usize,
	clear_term: bool,
	color_seed: u16,
	turn_duration: Duration,
}

impl Rules {
	pub fn clear_term(mut self, clear_term: bool) -> Self {
		self.clear_term = clear_term;
		self
	}

	pub fn color_seed(mut self, color_seed: u16) -> Self {
		self.color_seed = color_seed;
		self
	}

	pub fn turn_duration(mut self, turn_duration: Duration) -> Self {
		self.turn_duration = turn_duration;
		self
	}
}

impl Default for Rules {
	fn default() -> Self {
		Self {
			display: true,
			display_skips: 0,
			clear_term: true,
			color_seed: 35,
			turn_duration: Duration::ZERO,
		}
	}
}

pub struct GameManager {
	game: Game,
	controller: Box<dyn Controller>,
	grid_displayer: GridDisplayer,
	display_to_skip: usize,
	display: bool,
	display_skips: usize,
	clear_term: bool,
	turn_duration: Duration,
}

impl GameManager {
	pub fn new(
		game_rules: game::Rules,
		manager_rules: self::Rules,
		controller: Box<dyn Controller>,
	) -> Self {
		let game = Game::new(game_rules);
		let Rules {
			clear_term,
			color_seed,
			display,
			display_skips,
			turn_duration,
		} = manager_rules;
		let grid_displayer = GridDisplayer::new(color_seed);
		Self {
			game,
			controller,
			display_to_skip: 0,
			display,
			display_skips,
			clear_term,
			turn_duration,
			grid_displayer,
		}
	}

	pub fn turn(&mut self) -> Result<(), GameError> {
		if self.display {
			if self.display_to_skip == 0 {
				self.refresh_display();
				self.display_to_skip = self.display_skips;
			} else {
				self.display_to_skip -= 1;
			}
		}
		let movement = self.controller.next_move(self.game.get_board())?;
		self.game.turn(movement)?;
		thread::sleep(self.turn_duration);
		Ok(())
	}

	pub fn refresh_display(&self) {
		if self.clear_term {
			super::clear_term();
		}
		let grid = self.game.get_board();
		let text = self.grid_displayer.display(grid);
		println!("{text}");
	}

	pub fn play_all(&mut self) -> Result<(), GameError> {
		loop {
			self.turn()?;
		}
	}
}