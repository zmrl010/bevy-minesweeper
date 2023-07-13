pub mod components;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
pub use resources::board_options::BoardOptions;
use resources::tile_map::TileMap;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board);

        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    /// system to generate the complete board
    pub fn create_board(mut _commands: Commands, board_options: Option<Res<BoardOptions>>) {
        let options = match board_options {
            None => default(),
            Some(o) => o.clone(),
        };

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);

        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());
    }
}
