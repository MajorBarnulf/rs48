pub mod lib;

use lib::prelude::*;

fn main() -> Result<(), GameError> {
	let game_rules = GameRules::default().size(8).spawn_per_turn(1);
	let manager_rules = ManagerRules::default();
	let controller = PlayerController::new().into_box();
	let mut managed = GameManager::new(game_rules, manager_rules, controller);
	managed.play_all()
}
