use std::{fmt::Display, str::FromStr, time::Duration};

use clap::Parser;
use rs48_lib::prelude::*;

#[derive(Clone, Debug)]
pub enum ControllerParam {
	Player,
	Random,
	Simulated,
}

impl Display for ControllerParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match *self {
			ControllerParam::Player => "player",
			ControllerParam::Random => "random",
			ControllerParam::Simulated => "simulated",
		})
	}
}

impl FromStr for ControllerParam {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"player" => Ok(Self::Player),
			"random" => Ok(Self::Random),
			"simulated" => Ok(Self::Simulated),
			_ => Err(format!("failed to parse '{s}' into a 'ControllerParam'")),
		}
	}
}

/// Game of 2048 written in rust with a lot of configurations
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Arguments {
	/// size of the grid on which the game is played
	#[clap(short, long, default_value_t = 4)]
	size: usize,

	/// number of tiles that will spawn on the grid each turn
	#[clap(short = 'w', long, default_value_t = 1)]
	spawn: usize,

	/// disable clearing the terminal to refresh the screen
	#[clap(long)]
	no_clear: bool,

	/// skips the refresh of that many turns, allow AIs to play faster
	#[clap(short = 'k', long, default_value_t = 0)]
	display_skips: usize,

	/// delay in ms to add between each turns
	#[clap(short, long, default_value_t = 0)]
	delay: u64,

	/// the controller to use for the game
	#[clap(short, long, default_value_t = ControllerParam::Player)]
	controller: ControllerParam,

	/// sets a seed for the color pattern, 0 for random, default is 35
	#[clap(long, default_value_t = 35)]
	color_seed: u16,
}

fn main() -> Result<(), GameError> {
	let arguments = Arguments::parse();

	let game_rules = GameRules::default()
		.size(arguments.size)
		.spawn_per_turn(arguments.spawn);

	let color_seed = seed_or_random(arguments.color_seed);
	let manager_rules = ManagerRules::default()
		.clear_term(!arguments.no_clear)
		.color_seed(color_seed)
		.display_skips(arguments.display_skips)
		.turn_duration(Duration::from_millis(arguments.delay));

	let controller = match arguments.controller {
		ControllerParam::Player => PlayerController::default().into_box(),
		ControllerParam::Random => RandomController::default().into_box(),
		ControllerParam::Simulated => SimulatedController::new(100, 20).into_box(),
	};
	let mut managed = GameManager::new(game_rules, manager_rules, controller);

	managed.play_all()
}

fn seed_or_random(input: u16) -> u16 {
	if input == 0 {
		rand::random()
	} else {
		input
	}
}
