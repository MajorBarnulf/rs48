/// 0: '┘'
///
/// 1: '┐'
///
/// 2: '┌'
///
/// 3: '└'
///
/// 4: '┼'
///
/// 5: '─'
///
/// 6: '├'
///
/// 7: '┤'
///
/// 8: '┴'
///
/// 9: '┬'
///
/// 10: '│'
const DISPLAY_CHAR: [&'static str; 11] = ["┘", "┐", "┌", "└", "┼", "─", "├", "┤", "┴", "┬", "│"];

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

    const TILE_LENGTH: usize = 7;
    const TILE_HEIGHT: usize = 3;

    pub fn display(&self) -> String {
        match self.value {
            Some(value) => Self::display_number(value),
            None => [
                // empty tile
                "       ", "       ", "       ",
            ]
            .join("\n"),
        }
    }

    pub fn display_number(value: usize) -> String {
        let result = [
            // number tile
            "┌─────┐",
            &Self::pad_both(value.to_string(), Self::TILE_LENGTH),
            "└─────┘",
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
}

#[derive(Clone)]
pub struct Grid {
    size: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        let tiles = (0..size)
            .map(|_| (0..size).map(|_| Tile::new_empty()).collect())
            .collect();
        Self { size, tiles }
    }

    pub fn set(&mut self, (x, y): (usize, usize), value: Option<usize>) {
        self.tiles[y][x] = if let Some(value) = value {
            Tile::new_with_value(value)
        } else {
            Tile::new_empty()
        };
    }

    pub fn get(&self, (x, y): (usize, usize)) -> Option<&Tile> {
        match self.tiles.get(y).map(|row| row.get(x)) {
            Some(Some(tile)) => Some(tile),
            _ => None,
        }
    }

    pub fn get_val(&self, (x, y): (usize, usize)) -> Option<usize> {
        match self.get((x, y)).map(|tile| tile.value()) {
            Some(Some(value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &Tile {
        &mut self.tiles[y][x]
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn move_tile(&mut self, (src_x, src_y): (usize, usize), (dst_x, dst_y): (usize, usize)) {
        let src = self.tiles[src_y][src_x].clone();
        self.tiles[dst_y][dst_x] = src;
        self.tiles[src_y][src_x] = Tile::new_empty();
    }

    pub fn display(&self) -> String {
        let tiles: Vec<Vec<_>> = self
            .tiles
            .iter()
            .map(|row| row.iter().map(|tile| tile.display()).collect())
            .collect();
        let row_displays: Vec<_> = tiles
            .iter()
            .map(|row| {
                let mut row_lines = (0..Tile::TILE_HEIGHT).map(|_| vec![]).collect::<Vec<_>>();
                // push every item lines in [`row_lines`]
                for item in row {
                    item.split('\n')
                        .into_iter()
                        .zip(row_lines.iter_mut())
                        .for_each(|(item_line, row_line)| row_line.push(item_line.to_string()));
                }
                // join lines of [`row_lines`]
                let row_lines = row_lines
                    .iter_mut()
                    .map(|line_parts| line_parts.join(DISPLAY_CHAR[10]).to_string())
                    .map(|line| [DISPLAY_CHAR[10], &line, DISPLAY_CHAR[10]].join(""))
                    .collect::<Vec<_>>();
                row_lines.join("\n")
            })
            .collect();

        [
            self.first_grid_display_line(),
            row_displays.join(&self.between_grid_display_line()),
            self.last_grid_display_line(),
        ]
        .join("\n")
    }

    fn first_grid_display_line(&self) -> String {
        let middle = (0..self.size)
            .map(|_| DISPLAY_CHAR[5].repeat(Tile::TILE_LENGTH))
            .collect::<Vec<_>>()
            .join(DISPLAY_CHAR[9]);
        [DISPLAY_CHAR[2], &middle, DISPLAY_CHAR[1]].join("")
    }

    fn between_grid_display_line(&self) -> String {
        let middle = (0..self.size)
            .map(|_| DISPLAY_CHAR[5].repeat(Tile::TILE_LENGTH))
            .collect::<Vec<_>>()
            .join(DISPLAY_CHAR[4]);
        ["\n", DISPLAY_CHAR[6], &middle, DISPLAY_CHAR[7], "\n"].join("")
    }

    fn last_grid_display_line(&self) -> String {
        let middle = (0..self.size)
            .map(|_| DISPLAY_CHAR[5].repeat(Tile::TILE_LENGTH))
            .collect::<Vec<_>>()
            .join(DISPLAY_CHAR[8]);
        [DISPLAY_CHAR[3], &middle, DISPLAY_CHAR[0], "\n"].join("")
    }
}
