use std::{thread, time::Duration};

use crate::{
	controller::Controller,
	game::{self, Game, GameError},
};

use super::{clear_term, grid_displayer::GridDisplayer};

pub struct Rules {
	display: bool,
	display_skips: usize,
	clear_term: bool,
	color_seed: u16,
	turn_duration: Duration,
}

impl Rules {
	/// wether to display the game at all or not
	pub fn display(mut self, display: bool) -> Self {
		self.display = display;
		self
	}

	/// turns to skip the display of
	pub fn display_skips(mut self, display_skips: usize) -> Self {
		self.display_skips = display_skips;
		self
	}

	/// wether to clear the terminal or not between displays
	pub fn clear_term(mut self, clear_term: bool) -> Self {
		self.clear_term = clear_term;
		self
	}

	/// seed for the procedural coloration of tiles
	pub fn color_seed(mut self, color_seed: u16) -> Self {
		self.color_seed = color_seed;
		self
	}

	/// duration of pauses between turns
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
		self.display_conditionnally();
		self.game_turn()?;
		thread::sleep(self.turn_duration);
		Ok(())
	}

	fn display_conditionnally(&mut self) {
		if self.display {
			if self.display_to_skip == 0 {
				if self.clear_term {
					clear_term();
				}
				self.print_display();
				self.display_to_skip = self.display_skips;
			} else {
				self.display_to_skip -= 1;
			}
		}
	}

	fn game_turn(&mut self) -> Result<(), GameError> {
		let board = self.game.get_board();
		let movement = self.controller.next_move(board)?;
		self.game.turn(movement)?;
		Ok(())
	}

	pub fn print_display(&self) {
		let headline_display = self.get_headline_display();
		println!("{headline_display}");
		let grid = self.game.get_board();
		let grid_display = self.grid_displayer.display(grid);
		println!("{grid_display}");
	}

	fn get_headline_display(&self) -> String {
		let score = self.game.get_score();
		let turn = self.game.get_turn_index();
		let biggest_tile = self.game.get_board().biggest_value();
		format!("score: {score:>12} | biggest tile: {biggest_tile:>12} | turn: {turn:>12}")
	}

	pub fn play_all(&mut self) -> Result<(), GameError> {
		loop {
			self.turn()?;
		}
	}
}
