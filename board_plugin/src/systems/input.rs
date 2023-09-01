use crate::events::{TileMarkEvent, TileTriggerEvent};
use crate::Board;
use bevy::input::mouse::MouseButtonInput;
use bevy::log;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn handle_input(
    window_query: Query<&Window, With<PrimaryWindow>>,
    board: Res<Board>,
    mut button_event_reader: EventReader<MouseButtonInput>,
    mut tile_trigger_event_writer: EventWriter<TileTriggerEvent>,
    mut tile_mark_event_writer: EventWriter<TileMarkEvent>,
) {
    let Ok(window) = window_query.get_single() else {
        log::debug!("Window not found.");
        return;
    };

    let events = button_event_reader
        .iter()
        .filter(|event| event.state.is_pressed());

    for event in events {
        log::trace!("Mouse button pressed: {:?}", event.button);

        let Some(coordinates) = board.mouse_position(window) else {
            continue;
        };

        match event.button {
            MouseButton::Left => {
                log::info!("Trying to uncover tile on {}", coordinates);
                tile_trigger_event_writer.send(TileTriggerEvent(coordinates));
            }
            MouseButton::Right => {
                log::info!("Trying to mark tile on {}", coordinates);
                tile_mark_event_writer.send(TileMarkEvent(coordinates));
            }
            _ => (),
        }
    }
}
