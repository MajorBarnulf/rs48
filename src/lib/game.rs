use std::{error::Error, fmt::Display};

use super::{
    controller::{Controller, Move, PlayerController},
    grid::Grid,
};

pub struct Rules {
    size: usize,
    spawn_per_turn: usize,
    controller: Box<dyn Controller>,
}

impl Rules {
    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }

    pub fn spawn_per_turn(mut self, spawn_per_turn: usize) -> Self {
        self.spawn_per_turn = spawn_per_turn;
        self
    }
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            size: 4,
            spawn_per_turn: 1,
            controller: Box::new(PlayerController::new()),
        }
    }
}

#[derive(Debug)]
pub enum Err2048 {
    GridIsFull,
}

impl Display for Err2048 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            &Self::GridIsFull => "grid is full",
        };
        f.write_str(msg)
    }
}

impl Error for Err2048 {}

pub struct Game {
    board: Grid,
    controller: Box<dyn Controller>,
    spawn_per_turn: usize,
}

impl Game {
    pub fn new(rules: Rules) -> Self {
        let Rules {
            controller,
            size,
            spawn_per_turn,
        } = rules;

        Self {
            board: Grid::new(size),
            controller,
            spawn_per_turn,
        }
    }

    pub fn turn(&mut self) -> Result<(), Box<dyn Error>> {
        for _ in 0..self.spawn_per_turn {
            self.spawn_random()?;
        }
        self.refresh_display();
        let movement = self.controller.next_move(&self.board)?;
        self.perform_move(movement);
        Ok(())
    }

    pub fn spawn_random(&mut self) -> Result<(), Box<dyn Error>> {
        let mut potentials = vec![];
        for x in 0..self.board.size() {
            for y in 0..self.board.size() {
                if self.board.get((x, y)).unwrap().is_empty() {
                    potentials.push((x, y));
                }
            }
        }
        let potential_count = potentials.len() as f32;
        if potential_count == 0. {
            return Err(Err2048::GridIsFull.into());
        }
        let random = rand::random::<f32>() * potential_count;
        let index = random.floor() as usize;
        let (x, y) = potentials[index];
        self.board.set((x, y), Some(1));
        Ok(())
    }

    pub fn refresh_display(&self) {
        super::clear_term();
        let text = self.board.display();
        println!("{text}");
    }

    // TODO: macro peut être ?
    pub fn perform_move(&mut self, movement: Move) {
        match movement {
            Move::LEFT => {
                for y in 0..self.board.size() {
                    for x in 0..self.board.size() {
                        if !self.board.get((x, y)).unwrap().is_empty() {
                            self.perform_linear_move((-1, 0), (x, y));
                        }
                    }
                }
            }
            Move::RIGHT => {
                for y in 0..self.board.size() {
                    for x in (0..self.board.size()).rev() {
                        if !self.board.get((x, y)).unwrap().is_empty() {
                            self.perform_linear_move((1, 0), (x, y));
                        }
                    }
                }
            }
            Move::UP => {
                for x in 0..self.board.size() {
                    for y in 0..self.board.size() {
                        if !self.board.get((x, y)).unwrap().is_empty() {
                            self.perform_linear_move((0, -1), (x, y));
                        }
                    }
                }
            }
            Move::DOWN => {
                for x in 0..self.board.size() {
                    for y in (0..self.board.size()).rev() {
                        if !self.board.get((x, y)).unwrap().is_empty() {
                            self.perform_linear_move((0, 1), (x, y));
                        }
                    }
                }
            }
        };
    }

    fn perform_linear_move(&mut self, direction: (isize, isize), tile_pos: (usize, usize)) {
        let mut displacement = Displacement::new(&mut self.board, tile_pos, direction);
        displacement.move_all();
    }
}

pub struct Displacement<'g> {
    grid: &'g mut Grid,
    position: (usize, usize),
    direction: (isize, isize),
}

impl<'g> Displacement<'g> {
    pub fn new(grid: &'g mut Grid, position: (usize, usize), direction: (isize, isize)) -> Self {
        Self {
            grid,
            position,
            direction,
        }
    }

    pub fn move_all(&mut self) {
        loop {
            let can_continue = self.move_once();
            if !can_continue {
                break;
            }
        }
    }

    fn move_once(&mut self) -> bool {
        let current_pos = self.position.clone();
        let current_value = self.grid.get_val(current_pos).unwrap();
        if let Some(next_pos) = self.get_next_pos() {
            match self.grid.get_val(next_pos) {
                None => {
                    self.grid.move_tile(current_pos, next_pos);
                    self.set_pos(next_pos);
                    true
                }
                Some(value) if value == current_value => {
                    self.grid.move_tile(current_pos, next_pos);
                    self.grid.set(next_pos, Some(value * 2));
                    false
                }
                Some(_) => false,
            }
        } else {
            false
        }
    }

    fn get_next_pos(&self) -> Option<(usize, usize)> {
        let (current_x, current_y) = self.position.clone();
        let (dx, dy) = self.direction.clone();
        if would_overflow(current_x, dx, self.grid.size() - 1)
            || would_overflow(current_y, dy, self.grid.size() - 1)
        {
            None
        } else {
            let next_x = (current_x as isize) + dx;
            let next_y = (current_y as isize) + dy;
            Some((next_x as usize, next_y as usize))
        }
    }

    fn set_pos(&mut self, (x, y): (usize, usize)) {
        self.position = (x, y);
    }
}

fn would_overflow(n: usize, d: isize, max: usize) -> bool {
    let too_little = n == 0 && d == -1;
    let too_big = n == max && d == 1;
    too_little || too_big
}
