use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyHealth {
    pub current: i32,
    pub max: i32,
}

impl EnemyHealth {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }
}
