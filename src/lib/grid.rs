use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
	mem::transmute_copy,
};

use termion::color;

#[derive(Clone, Copy)]
pub struct Tile {
	value: Option<usize>,
}

impl Tile {
	pub fn new_with_value(value: usize) -> Self {
		Self { value: Some(value) }
	}
	pub fn new_empty() -> Self {
		Self { value: None }
	}

	pub fn value(&self) -> Option<usize> {
		self.value.clone()
	}

	pub fn is_empty(&self) -> bool {
		if let Some(_) = self.value {
			false
		} else {
			true
		}
	}
}

///
/// displayability
///
impl Tile {
	const TILE_LENGTH: usize = 7;
	const TILE_HEIGHT: usize = 3;

	pub fn display(&self) -> String {
		match self.value {
			Some(value) => Self::color_representation(Self::display_number(value), value),
			None => [
				// empty tile
				"       ", "       ", "       ",
			]
			.join("\n"),
		}
	}

	fn display_number(value: usize) -> String {
		let result = [
			// number tile
			"┌─   ─┐",
			&Self::pad_both(value.to_string(), Self::TILE_LENGTH),
			"└─   ─┘",
		]
		.join("\n");
		result
	}

	fn pad_both(text: String, length: usize) -> String {
		let mut text = text;
		while text.len() < length {
			text = format!(" {text} ");
		}
		if text.len() > length {
			(&text)[..length].to_string()
		} else {
			text
		}
	}

	fn color_representation(text: String, value: usize) -> String {
		let color = Self::hashed_color(value);
		let color_code = color::Bg(color);
		let reset_code = color::Bg(color::Reset);

		let text = text
			.split("\n")
			.map(|line| format!("{color_code}{line}{reset_code}"))
			.collect::<Vec<_>>()
			.join("\n");
		text
	}

	// [  |  |  ]

	fn hashed_color(value: usize) -> color::Rgb {
		let mut hasher = DefaultHasher::new();
		value.hash(&mut hasher);
		let hash = hasher.finish();
		// SAFETY:
		// there are no logic that relies on the value of the outputted numbers, thus it is safe to create them without constructors
		let [frac_a, frac_b]: [f64; 2] = unsafe { transmute_copy::<_, [u32; 2]>(&hash) }
			.into_iter()
			.map(|frac| (frac as f64) / (u32::MAX as f64))
			.collect::<Vec<_>>()
			.try_into()
			.unwrap();

		let mut remaining = 300f64;
		let r = Self::take_fraction(&mut remaining, frac_a, 200.) as u8;
		let g = Self::take_fraction(&mut remaining, frac_b, 200.) as u8;
		let b = remaining as u8;
		color::Rgb(r, g, b)
	}

	fn take_fraction(remainder: &mut f64, frac: f64, max: f64) -> f64 {
		let result = (*remainder * frac).min(max);
		*remainder -= result;
		result
	}
}

#[derive(Clone)]
pub struct Grid {
	size: usize,
	tiles: Vec<Vec<Tile>>,
}

impl Grid {
	///
	/// constructor
	///
	pub fn new(size: usize) -> Self {
		let tiles = (0..size)
			.map(|_| (0..size).map(|_| Tile::new_empty()).collect())
			.collect();
		Self { size, tiles }
	}

	///
	/// set the value of the tile at the selected position
	///
	pub fn set(&mut self, (x, y): (usize, usize), value: Option<usize>) {
		self.tiles[y][x] = if let Some(value) = value {
			Tile::new_with_value(value)
		} else {
			Tile::new_empty()
		};
	}

	///
	/// get a tile if the position is in the grid
	///
	pub fn get(&self, (x, y): (usize, usize)) -> Option<&Tile> {
		match self.tiles.get(y).map(|row| row.get(x)) {
			Some(Some(tile)) => Some(tile),
			_ => None,
		}
	}

	///
	/// get the value of a tile if the position is in the grid and the tile has a value
	///
	pub fn get_val(&self, (x, y): (usize, usize)) -> Option<usize> {
		match self.get((x, y)).map(|tile| tile.value()) {
			Some(Some(value)) => Some(value),
			_ => None,
		}
	}

	///
	/// get the size of the grid
	///
	pub fn size(&self) -> usize {
		self.size
	}

	///
	/// move a tile over another one, replace the previously occupied place by an empty tile and overrides the destination
	///
	pub fn move_tile(&mut self, (src_x, src_y): (usize, usize), (dst_x, dst_y): (usize, usize)) {
		let src = self.tiles[src_y][src_x].clone();
		self.tiles[dst_y][dst_x] = src;
		self.tiles[src_y][src_x] = Tile::new_empty();
	}
}

///
/// displayability
///
impl Grid {
	/// (0: '┘'), (1: '┐'), (2: '┌'), (3: '└'), (4: '┼'), (5: '─'), (6: '├'), (7: '┤'), (8: '┴'), (9: '┬'), (10: '│')
	const DISPLAY_CHAR: [&'static str; 11] =
		["┘", "┐", "┌", "└", "┼", "─", "├", "┤", "┴", "┬", "│"];

	///
	/// returns a string of multiple lines representing the grid
	///
	pub fn display(&self) -> String {
		let tiles: Vec<Vec<_>> = self
			.tiles
			.iter()
			.map(|row| row.iter().map(|tile| tile.display()).collect())
			.collect();
		let row_representations: Vec<_> = tiles
			.iter()
			.map(|row_representation| {
				let mut row_lines = (0..Tile::TILE_HEIGHT).map(|_| vec![]).collect::<Vec<_>>();
				// push every item lines in [`row_lines`]
				for item_representation in row_representation {
					item_representation
						.split('\n')
						.into_iter()
						.zip(row_lines.iter_mut())
						.for_each(|(item_line, row_line)| row_line.push(item_line.to_string()));
				}
				// join lines of [`row_lines`]
				let row_lines = row_lines
					.iter_mut()
					.map(|line_parts| line_parts.join(Self::DISPLAY_CHAR[10]).to_string())
					.map(|line| [Self::DISPLAY_CHAR[10], &line, Self::DISPLAY_CHAR[10]].join(""))
					.collect::<Vec<_>>();
				row_lines.join("\n")
			})
			.collect();

		[
			self.first_grid_display_line(),
			row_representations.join(&self.between_grid_display_line()),
			self.last_grid_display_line(),
		]
		.join("\n")
	}

	fn first_grid_display_line(&self) -> String {
		let middle = (0..self.size)
			.map(|_| Self::DISPLAY_CHAR[5].repeat(Tile::TILE_LENGTH))
			.collect::<Vec<_>>()
			.join(Self::DISPLAY_CHAR[9]);
		[Self::DISPLAY_CHAR[2], &middle, Self::DISPLAY_CHAR[1]].join("")
	}

	fn between_grid_display_line(&self) -> String {
		let middle = (0..self.size)
			.map(|_| Self::DISPLAY_CHAR[5].repeat(Tile::TILE_LENGTH))
			.collect::<Vec<_>>()
			.join(Self::DISPLAY_CHAR[4]);
		[
			"\n",
			Self::DISPLAY_CHAR[6],
			&middle,
			Self::DISPLAY_CHAR[7],
			"\n",
		]
		.join("")
	}

	fn last_grid_display_line(&self) -> String {
		let middle = (0..self.size)
			.map(|_| Self::DISPLAY_CHAR[5].repeat(Tile::TILE_LENGTH))
			.collect::<Vec<_>>()
			.join(Self::DISPLAY_CHAR[8]);
		[Self::DISPLAY_CHAR[3], &middle, Self::DISPLAY_CHAR[0], "\n"].join("")
	}
}
