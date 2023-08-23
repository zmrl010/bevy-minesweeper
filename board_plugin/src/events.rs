use crate::components::Coordinates;
use bevy::prelude::Event;

/// Event that occurs when a tile is triggered (left clicked)
#[derive(Debug, Copy, Clone, Event)]
pub struct TileTriggerEvent(pub Coordinates);

/// Event that occurs when the board is completed
#[derive(Debug, Copy, Clone, Event)]
pub struct BoardCompletedEvent;

/// Event that occurs when a player uncovers a bomb
#[derive(Debug, Copy, Clone, Event)]
pub struct BombExplosionEvent;

/// Event that occurs when a tile is marked (right clicked)
#[derive(Debug, Copy, Clone, Event)]
pub struct TileMarkEvent(pub Coordinates);
