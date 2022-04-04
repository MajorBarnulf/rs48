use lib::{
	controller::{player::PlayerController, Controller},
	game::{Game, Rules},
};

pub mod lib;

fn main() {
	let rules = Rules::default()
		.size(4)
		.spawn_per_turn(1)
		.clear_term(false);
	let controller = PlayerController::new().into_box();
	let mut game = Game::new(rules).controlled(controller);
	loop {
		game.turn().unwrap();
	}
}
