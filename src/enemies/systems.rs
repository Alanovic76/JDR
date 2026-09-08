use bevy::prelude::*;

use super::components::{Enemy, EnemyHealth};

/// Fait apparaître un slime de test. À terme, ce système sera piloté par
/// la carte chargée (points de spawn définis dans assets/maps/).
pub fn spawn_enemies(mut commands: Commands) {
    commands.spawn((
        Enemy,
        EnemyHealth::new(10),
        Sprite::from_color(Color::srgb(0.8, 0.2, 0.2), Vec2::new(16.0, 16.0)),
        Transform::from_xyz(64.0, 64.0, 10.0),
    ));
}
