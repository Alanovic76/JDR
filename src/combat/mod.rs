mod components;
mod systems;

use bevy::prelude::*;

pub use components::*;

use systems::attack_input_system;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>()
            .add_systems(Update, attack_input_system);
    }
}
