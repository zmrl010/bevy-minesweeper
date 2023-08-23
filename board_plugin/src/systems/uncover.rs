use crate::events::{BoardCompletedEvent, BombExplosionEvent, TileTriggerEvent};
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
    mut board_completed_event_writer: EventWriter<BoardCompletedEvent>,
    mut bomb_explosion_event_writer: EventWriter<BombExplosionEvent>,
) {
    for (entity, parent) in children.iter() {
        commands.entity(entity).despawn_recursive();

        let Ok((coords, bomb, bomb_counter)) = parents.get(parent.get()) else {
            log::error!("Parent not found!");
            continue;
        };

        match board.try_uncover_tile(coords) {
            Some(e) => log::debug!("Uncovered tile {} (entity: {:?})", coords, e),
            None => log::debug!("Tried to uncover an already uncovered tile"),
        };

        if board.is_completed() {
            log::info!("Board completed");
            board_completed_event_writer.send(BoardCompletedEvent);
        }

        if bomb.is_some() {
            log::info!("Boom!");
            bomb_explosion_event_writer.send(BombExplosionEvent);
        } else if bomb_counter.is_none() {
            for entity in board.adjacent_covered_tiles(*coords) {
                commands.entity(entity).insert(Uncover);
            }
        }
    }
}
