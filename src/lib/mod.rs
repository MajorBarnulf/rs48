pub mod controller;
pub mod game;
pub mod grid;

pub fn clear_term() {
    print!("\x1B[2J\x1B[1;1H");
}
