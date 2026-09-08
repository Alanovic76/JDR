mod components;
mod systems;

use bevy::prelude::*;

pub use components::*;

use systems::{movement_system, spawn_player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, movement_system);
    }
}
