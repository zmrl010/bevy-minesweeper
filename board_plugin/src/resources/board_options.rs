use bevy::prelude::{default, Resource, Vec3};
use serde::{Deserialize, Serialize};

/// Tile size options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize {
    /// Fixed tile size
    Fixed(f32),
    /// Window adaptive tile size
    Adaptive { min: f32, max: f32 },
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive {
            min: 10.0,
            max: 50.0,
        }
    }
}

/// Board position customization options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoardPosition {
    Centered { offset: Vec3 },
    Custom(Vec3),
}

impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered { offset: default() }
    }
}

/// Board generation options
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct BoardOptions {
    /// Tile map size (width, height)
    pub map_size: (u16, u16),
    /// Number of bombs spawned
    pub bomb_count: u16,
    /// Board world position
    pub position: BoardPosition,
    /// Size of each individual tile
    pub tile_size: TileSize,
    /// Padding between tiles
    pub tile_padding: f32,
    /// Does the board generate a safe place to start
    pub safe_start: bool,
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            map_size: (15, 15),
            bomb_count: 30,
            position: default(),
            tile_size: default(),
            tile_padding: 0.,
            safe_start: false,
        }
    }
}
