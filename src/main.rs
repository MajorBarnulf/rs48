use lib::{
	controller::{player::PlayerController, random::RandomController, Controller},
	game::{self, GameError},
	game_manager::{self, GameManager},
};

pub mod lib;

fn main() -> Result<(), GameError> {
	let game_rules = game::Rules::default().size(8).spawn_per_turn(1);
	let manager_rules = game_manager::Rules::default();
	let controller = PlayerController::new().into_box();
	let mut managed = GameManager::new(game_rules, manager_rules, controller);
	managed.play_all()
}
