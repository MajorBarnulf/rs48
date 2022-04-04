pub mod controller;
pub mod game;
pub mod game_manager;
pub mod grid;
pub mod grid_displayer;

pub fn clear_term() {
	print!("\x1B[2J\x1B[1;1H");
}

pub mod prelude {
	pub use super::controller::{
		Controller, PlayerController, RandomController, SimulatedController,
	};
	pub use super::game::GameError;
	pub use super::game::Rules as GameRules;
	pub use super::game_manager::GameManager;
	pub use super::game_manager::Rules as ManagerRules;
}
