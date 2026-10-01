mod components;
mod systems;

use bevy::prelude::*;

pub use components::*;
use systems::combat_system;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CombatState>()
            .init_resource::<DiceRng>()
            .add_systems(Update, combat_system);
    }
}
