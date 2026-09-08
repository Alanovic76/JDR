use bevy::prelude::*;

#[derive(Component)]
pub struct Attack {
    pub damage: i32,
}

impl Default for Attack {
    fn default() -> Self {
        Self { damage: 3 }
    }
}

/// Émis quand le joueur déclenche une attaque. Les systèmes d'ennemis
/// écoutent cet event pour résoudre les dégâts.
#[derive(Event)]
pub struct AttackEvent {
    pub attacker: Entity,
}
