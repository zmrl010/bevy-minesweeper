use crate::bounds::Bounds2;
use crate::{Coordinates, TileMap};
use bevy::utils::HashMap;
use bevy::{log, prelude::*};

#[derive(Debug, Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub entity: Entity,
    pub covered_tiles: HashMap<Coordinates, Entity>,
    pub marked_tiles: Vec<Coordinates>,
}

impl Board {
    /// Translate mouse position to board coordinates
    pub fn mouse_position(&self, window: &Window) -> Option<Coordinates> {
        let Some(position) = window.cursor_position() else {
            return None;
        };
        log::trace!("Mouse position: {}", position);

        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2.;
        log::trace!("Adjusted position: {}", position);

        if !self.bounds.in_bounds(position) {
            return None;
        }
        log::trace!("In bounds {:?}", self.bounds);

        let coordinates = position - self.bounds.position;

        Some(Coordinates {
            x: (coordinates.x / self.tile_size) as u16,
            y: (coordinates.y / self.tile_size) as u16,
        })
    }

    /// Retrieve a covered tile entity
    pub fn tile_to_uncover(&self, coords: &Coordinates) -> Option<&Entity> {
        if self.marked_tiles.contains(coords) {
            None
        } else {
            self.covered_tiles.get(coords)
        }
    }

    /// Attempt to uncover a tile, returning the entity
    pub fn try_uncover_tile(&mut self, coords: &Coordinates) -> Option<Entity> {
        if self.marked_tiles.contains(coords) {
            self.unmark_tile(coords)?;
        }
        self.covered_tiles.remove(coords)
    }

    pub fn try_toggle_mark(
        &mut self,
        coords: &Coordinates,
    ) -> Option<(Entity, bool)> {
        let entity = *self.covered_tiles.get(coords)?;
        let mark = self.marked_tiles.contains(coords);

        if mark {
            self.unmark_tile(coords)?;
        } else {
            self.marked_tiles.push(*coords);
        };

        Some((entity, mark))
    }

    /// Retrieve adjacent covered tile entities of `coord`
    pub fn adjacent_covered_tiles(&self, coords: Coordinates) -> Vec<Entity> {
        self.tile_map
            .safe_square_at(coords)
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect()
    }

    /// Removes the `coords` from `marked_tiles`
    pub fn unmark_tile(&mut self, coords: &Coordinates) -> Option<Coordinates> {
        self.marked_tiles
            .iter()
            .position(|a| a == coords)
            .and_then(|pos| Some(self.marked_tiles.remove(pos)))
            .or_else(|| {
                log::error!("Failed to unmark tile at {}", coords);
                None
            })
    }

    /// Is the board complete
    pub fn is_completed(&self) -> bool {
        self.tile_map.bomb_count() as usize == self.covered_tiles.len()
    }
}
