use crate::components::Coordinates;
use crate::resources::tile::Tile;
use rand::{thread_rng, Rng};
use std::ops::{Deref, DerefMut};

/// Delta coordinates for all 8 square neighbors
const SQUARE_COORDINATES: [(i8, i8); 8] = [
    // Bottom left
    (-1, -1),
    // Bottom
    (0, -1),
    // Bottom right
    (1, -1),
    // Left
    (-1, 0),
    // Right
    (1, 0),
    // Top Left
    (-1, 1),
    // Top
    (0, 1),
    // Top right
    (1, 1),
];

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

    pub fn safe_square_at(&self, coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |tuple| coordinates + tuple)
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

        format!("{buffer}{line}")
    }

    pub fn is_bomb_at(&self, coordinates: Coordinates) -> bool {
        if coordinates.x >= self.width || coordinates.y >= self.height {
            return false;
        };
        self.map[coordinates.y as usize][coordinates.x as usize].is_bomb()
    }

    pub fn bomb_count_at(&self, coordinates: Coordinates) -> u8 {
        if self.is_bomb_at(coordinates) {
            return 0;
        }

        self.safe_square_at(coordinates)
            .filter(|coord| self.is_bomb_at(*coord))
            .count() as u8
    }

    pub fn set_bombs(&mut self, bomb_count: u16) {
        self.bomb_count = bomb_count;

        let mut remaining_bombs = bomb_count;
        let mut rng = thread_rng();

        while remaining_bombs > 0 {
            let x = rng.gen_range(0..self.width) as usize;
            let y = rng.gen_range(0..self.height) as usize;

            if let Tile::Empty = self[x][y] {
                self[y][x] = Tile::Bomb;
                remaining_bombs -= 1;
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let coords = Coordinates { x, y };

                if self.is_bomb_at(coords) {
                    continue;
                }

                let bomb_count = self.bomb_count_at(coords);
                if bomb_count == 0 {
                    continue;
                }

                let tile = &mut self[y as usize][x as usize];

                *tile = Tile::BombNeighbor(bomb_count);
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
