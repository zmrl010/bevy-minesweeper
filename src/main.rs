use bevy::{log, prelude::*};
use board_plugin::{BoardOptions, BoardPlugin};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    InGame,
    Out,
}

fn main() {
    let mut app = App::new();

    app.add_state::<AppState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mine Sweeper".to_string(),
                resolution: (700.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(BoardOptions {
            map_size: (20, 20),
            bomb_count: 40,
            tile_padding: 3.0,
            ..default()
        })
        .add_plugins(BoardPlugin {
            running_state: AppState::InGame,
        })
        .add_systems(Startup, camera_setup)
        .add_systems(Update, update_state);

    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}

fn update_state(
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
