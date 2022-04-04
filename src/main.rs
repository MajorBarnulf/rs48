use lib::game::{Game, Rules};

pub mod lib;

fn main() {
	let rules = Rules::default().size(4).spawn_per_turn(1);
	let mut game = Game::new(rules);
	loop {
		game.turn().unwrap();
	}
}
