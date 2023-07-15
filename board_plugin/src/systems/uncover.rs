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
    }
}
