use bevy::prelude::*;
use board_plugin::{BoardOptions, BoardPlugin};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Mine Sweeper".to_string(),
            resolution: (700.0, 800.0).into(),
            ..default()
        }),
        ..default()
    }));

    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.0,
        ..default()
    });

    app.add_plugins(BoardPlugin);

    app.add_systems(Startup, camera_setup);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}
