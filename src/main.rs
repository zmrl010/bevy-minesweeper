#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    log::{self, Level, LogPlugin},
    prelude::*,
};
use board_plugin::{
    resources::{BoardAssets, BoardPosition, SpriteMaterial},
    BoardOptions, BoardPlugin,
};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    InGame,
    #[default]
    Out,
}

fn main() {
    let mut app = App::new();

    app.add_state::<AppState>()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Minesweeper".to_string(),
                        resolution: (700.0, 800.0).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    #[cfg(debug_assertions)]
                    level: Level::TRACE,
                    ..default()
                }),
            BoardPlugin {
                running_state: AppState::InGame,
            },
        ))
        .add_systems(Startup, (setup_camera, setup_board))
        .add_systems(Update, handle_input);

    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.run();
}

fn setup_board(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 50,
        tile_padding: 1.0,
        safe_start: true,
        position: BoardPosition::Centered {
            offset: Vec3::new(0., 25., 0.),
        },
        ..default()
    });
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::GRAY,
            ..default()
        },
        bomb_counter_font: asset_server.load("fonts/pixeled.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: SpriteMaterial {
            color: Color::WHITE,
            texture: asset_server.load("sprites/flag.png"),
        },
        bomb_material: SpriteMaterial {
            color: Color::WHITE,
            texture: asset_server.load("sprites/bomb.png"),
        },
    });

    next_state.set(AppState::InGame);
}

fn setup_camera(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}

fn handle_input(
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");

        if state.get() == &AppState::InGame {
            log::info!("clearing game");

            next_state.set(AppState::Out);
        }
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");

        if state.get() == &AppState::Out {
            log::info!("loading game");

            next_state.set(AppState::InGame);
        }
    }
}
