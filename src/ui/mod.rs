mod components;
mod systems;

use bevy::prelude::*;

pub use components::*;

use systems::{spawn_hud, update_health_text};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud)
            .add_systems(Update, update_health_text);
    }
}
