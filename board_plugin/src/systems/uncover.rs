use crate::events::TileTriggerEvent;
use crate::{Board, Bomb, BombNeighbor, Coordinates, Uncover};
use bevy::log;
use bevy::prelude::*;

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_event_reader: EventReader<TileTriggerEvent>,
) {
    for trigger_event in tile_trigger_event_reader.iter() {
        if let Some(entity) = board.tile_to_uncover(&trigger_event.0) {
            commands.entity(*entity).insert(Uncover);
        }
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbor>)>,
) {
    for (entity, parent) in children.iter() {
        commands.entity(entity).despawn_recursive();

        let (coords, bomb, bomb_counter) = match parents.get(parent.get()) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{e}");
                continue;
            }
        };

        match board.try_uncover_tile(coords) {
            Some(e) => log::debug!("Uncovered tile {} (entity: {:?})", coords, e),
            None => log::debug!("Tried to uncover an already uncovered tile"),
        };

        if bomb.is_some() {
            log::info!("Boom!");
            todo!("Add explosion event")
        } else if bomb_counter.is_none() {
            for entity in board.adjacent_covered_tiles(*coords) {
                commands.entity(entity).insert(Uncover);
            }
        }
    }
}
