use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
	mem::transmute_copy,
};

use termion::color;

use super::grid::{Grid, Tile};

pub struct TileDisplayer {
	color_seed: u16,
}

impl TileDisplayer {
	pub fn new(color_seed: u16) -> Self {
		Self { color_seed }
	}

	const TILE_LENGTH: usize = 7;
	const TILE_HEIGHT: usize = 3;

	pub fn display(&self, tile: &Tile) -> String {
		match tile.value() {
			Some(value) => {
				Self::color_representation(Self::display_number(value), value, self.color_seed)
			}
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

	fn color_representation(text: String, value: usize, color_seed: u16) -> String {
		let color = Self::hashed_color(value, color_seed);
		let color_code = color::Bg(color);
		let reset_code = color::Bg(color::Reset);

		let text = text
			.split("\n")
			.map(|line| format!("{color_code}{line}{reset_code}"))
			.collect::<Vec<_>>()
			.join("\n");
		text
	}

	fn hashed_color(value: usize, color_seed: u16) -> color::Rgb {
		let mut hasher = DefaultHasher::new();
		(value + color_seed as usize).hash(&mut hasher);
		let hash = hasher.finish();
		// SAFETY:
		// there are no logic that relies on the value of the outputted numbers, thus it is safe to create them without constructors
		let [frac_a, frac_b]: [f64; 2] = unsafe { transmute_copy::<_, [u32; 2]>(&hash) }
			.into_iter()
			.map(|frac| (frac as f64) / (u32::MAX as f64))
			.collect::<Vec<_>>()
			.try_into()
			.unwrap();

		let mut remaining = 255f64;
		let r = Self::take_fraction(&mut remaining, frac_a, 150.) as u8;
		let g = Self::take_fraction(&mut remaining, frac_b, 150.) as u8;
		let b = remaining as u8;
		color::Rgb(r, g, b)
	}

	fn take_fraction(remainder: &mut f64, frac: f64, max: f64) -> f64 {
		let result = (*remainder * frac).min(max);
		*remainder -= result;
		result
	}
}

pub struct GridDisplayer {
	tile_displayer: TileDisplayer,
}

impl GridDisplayer {
	pub fn new(color_seed: u16) -> Self {
		let tile_displayer = TileDisplayer::new(color_seed);
		Self { tile_displayer }
	}

	/// (0: '┘'), (1: '┐'), (2: '┌'), (3: '└'), (4: '┼'), (5: '─'), (6: '├'), (7: '┤'), (8: '┴'), (9: '┬'), (10: '│')
	const DISPLAY_CHAR: [&'static str; 11] =
		["┘", "┐", "┌", "└", "┼", "─", "├", "┤", "┴", "┬", "│"];

	///
	/// returns a string of multiple lines representing the grid
	///
	pub fn display(&self, grid: &Grid) -> String {
		let tiles: Vec<Vec<_>> = grid
			.tiles()
			.iter()
			.map(|row| {
				row.iter()
					.map(|tile| self.tile_displayer.display(tile))
					.collect()
			})
			.collect();
		let row_representations: Vec<_> = tiles
			.iter()
			.map(|row_representation| {
				let mut row_lines = (0..TileDisplayer::TILE_HEIGHT)
					.map(|_| vec![])
					.collect::<Vec<_>>();
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
			self.first_grid_display_line(grid),
			row_representations.join(&self.between_grid_display_line(grid)),
			self.last_grid_display_line(grid),
		]
		.join("\n")
	}

	fn first_grid_display_line(&self, grid: &Grid) -> String {
		let middle = (0..grid.size())
			.map(|_| Self::DISPLAY_CHAR[5].repeat(TileDisplayer::TILE_LENGTH))
			.collect::<Vec<_>>()
			.join(Self::DISPLAY_CHAR[9]);
		[Self::DISPLAY_CHAR[2], &middle, Self::DISPLAY_CHAR[1]].join("")
	}

	fn between_grid_display_line(&self, grid: &Grid) -> String {
		let middle = (0..grid.size())
			.map(|_| Self::DISPLAY_CHAR[5].repeat(TileDisplayer::TILE_LENGTH))
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

	fn last_grid_display_line(&self, grid: &Grid) -> String {
		let middle = (0..grid.size())
			.map(|_| Self::DISPLAY_CHAR[5].repeat(TileDisplayer::TILE_LENGTH))
			.collect::<Vec<_>>()
			.join(Self::DISPLAY_CHAR[8]);
		[Self::DISPLAY_CHAR[3], &middle, Self::DISPLAY_CHAR[0], "\n"].join("")
	}
}
