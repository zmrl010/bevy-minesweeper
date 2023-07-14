use crate::Board;
use bevy::input::{mouse::MouseButtonInput, ButtonState};
use bevy::log;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn input_handling(
    window_query: Query<&Window, With<PrimaryWindow>>,
    board: Res<Board>,
    mut button_reader: EventReader<MouseButtonInput>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    for event in button_reader.iter() {
        if let ButtonState::Pressed = event.state {
            let position = window.cursor_position();
            if let Some(pos) = position {
                log::trace!("Mouse button pressed: {:?} at {}", event.button, pos);
                let tile_coordinates = board.mouse_position(window, pos);
                if let Some(coordinates) = tile_coordinates {
                    match event.button {
                        MouseButton::Left => {
                            log::info!("Trying to uncover tile on {}", coordinates);
                            todo!("generate an event")
                        }
                        MouseButton::Right => {
                            log::info!("Trying to mark tile on {}", coordinates);
                            todo!("generate an event")
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
