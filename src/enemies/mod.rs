mod components;
mod systems;

use bevy::prelude::*;

pub use components::*;

use systems::spawn_enemies;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies);
    }
}
