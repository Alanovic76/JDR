mod components;
mod systems;
use bevy::prelude::*;
pub use components::*;
use systems::combat_system;
pub struct CombatPlugin;
impl Plugin for CombatPlugin { fn build(&self,app:&mut App){ app.init_resource::<Score>().insert_resource(HighScores::load()).init_resource::<CombatState>().init_resource::<DiceRng>().init_resource::<Equipment>().init_resource::<TreasureState>().add_systems(Update,combat_system); } }
