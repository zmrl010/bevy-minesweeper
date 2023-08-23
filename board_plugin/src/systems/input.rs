use crate::events::{TileMarkEvent, TileTriggerEvent};
use crate::Board;
use bevy::input::{mouse::MouseButtonInput, ButtonState};
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

    for event in button_event_reader.iter() {
        if let ButtonState::Pressed = event.state {
            let position = window.cursor_position();
            if let Some(pos) = position {
                log::trace!("Mouse button pressed: {:?} at {}", event.button, pos);
                let tile_coordinates = board.mouse_position(window, pos);
                if let Some(coordinates) = tile_coordinates {
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
        }
    }
}
