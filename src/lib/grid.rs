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
	/// get the array of tiles
	///
	pub fn tiles(&self) -> &Vec<Vec<Tile>> {
		&self.tiles
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
