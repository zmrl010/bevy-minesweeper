pub mod components;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
pub use resources::board_options::BoardOptions;
use resources::tile_map::TileMap;

use crate::components::Coordinates;
use crate::resources::BoardPosition;
use crate::resources::TileSize;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board);

        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    /// system to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        let options = board_options.map(|o| o.clone()).unwrap_or_default();

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);

        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let Ok(window) = window_query.get_single() else {
            todo!();
        };

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive { min, max } => {
                Self::adaptive_tile_size(window, (min, max), (tile_map.width(), tile_map.height()))
            }
        };

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size: {board_size}");

        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p,
        };

        commands
            .spawn_empty()
            .insert(Name::new("Board"))
            .insert(SpatialBundle {
                visibility: Visibility::Visible,
                transform: Transform::from_translation(board_position),
                ..default()
            })
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..default()
                    })
                    .insert(Name::new("Background"));

                for (y, line) in tile_map.iter().enumerate() {
                    for (x, _tile) in line.iter().enumerate() {
                        parent
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::GRAY,
                                    custom_size: Some(Vec2::splat(
                                        tile_size - options.tile_padding as f32,
                                    )),
                                    ..default()
                                },
                                transform: Transform::from_xyz(
                                    (x as f32 * tile_size) + (tile_size / 2.),
                                    (y as f32 * tile_size) + (tile_size / 2.),
                                    1.,
                                ),
                                ..default()
                            })
                            .insert(Name::new(format!("Tile ({x}, {y})")))
                            .insert(Coordinates {
                                x: x as u16,
                                y: y as u16,
                            });
                    }
                }
            });
    }

    /// Compute a tile size that matches the window according to the tile map size
    fn adaptive_tile_size(
        window: &Window,
        (min, max): (f32, f32),
        (width, height): (u16, u16),
    ) -> f32 {
        let max_width = window.width() / width as f32;
        let max_height = window.height() / height as f32;

        max_width.min(max_height).clamp(min, max)
    }
}
