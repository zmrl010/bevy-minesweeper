use std::ops::{Deref, DerefMut};

use crate::components::Coordinates;
use crate::resources::tile::Tile;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct TileMap {
    bomb_count: u16,
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    /// Generate an empty map
    pub fn empty(width: u16, height: u16) -> Self {
        let map = (0..height)
            .into_iter()
            .map(|_| (0..width).into_iter().map(|_| Tile::Empty).collect())
            .collect();

        Self {
            bomb_count: 0,
            height,
            width,
            map,
        }
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let Self {
            width,
            height,
            bomb_count,
            ..
        } = self;
        let mut buffer = format!("Map ({width}, {height}) with {bomb_count} bombs:\n");

        let line: String = (0..=(self.width + 1)).into_iter().map(|_| '-').collect();
        buffer = format!("{buffer}{line}\n");

        for line in self.iter().rev() {
            buffer = format!("{buffer}|");
            for tile in line.iter() {
                buffer = format!("{buffer}{}", tile.console_output());
            }
            buffer = format!("{buffer}|\n");
        }

        format!("{}{}", buffer, line)
    }

    pub fn set_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;
        let mut remaining_bombs = bomb_count;
        let mut rng = thread_rng();

        while remaining_bombs > 0 {
            let (x, y) = (
                rng.gen_range(0..self.width) as usize,
                rng.gen_range(0..self.height) as usize,
            );
            if let Tile::Empty = self[x][y] {
                self[y][x] = Tile::Bomb;
                remaining_bombs -= 1;
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let coords = Coordinates { x, y };
            }
        }
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
