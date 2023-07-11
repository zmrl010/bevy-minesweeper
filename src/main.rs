use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: Window {
            title: "Mine Sweeper".to_string(),
            resolution: (700.0, 800.0).into(),
            ..Default::default()
        },
        ..Default::default()
    }));

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_startup_system(camera_setup);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}
