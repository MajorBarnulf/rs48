pub mod controller;
pub mod game;
pub mod game_manager;
pub mod grid;
pub mod grid_displayer;

pub fn clear_term() {
	print!("\x1B[2J\x1B[1;1H");
}
