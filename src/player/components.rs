use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Default for Health {
    fn default() -> Self {
        Self { current: 20, max: 20 }
    }
}
