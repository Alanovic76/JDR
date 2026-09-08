use bevy::prelude::*;

use super::components::{Health, Player, Speed};

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Speed(150.0),
        Health::default(),
        Sprite::from_color(Color::srgb(0.9, 0.9, 0.2), Vec2::new(16.0, 16.0)),
        Transform::from_xyz(0.0, 0.0, 10.0),
    ));
}

pub fn movement_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Speed, &mut Transform), With<Player>>,
) {
    let Ok((speed, mut transform)) = query.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    // ZQSD (clavier français) et flèches
    if keyboard.pressed(KeyCode::KeyZ) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyQ) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        let movement = direction.normalize() * speed.0 * time.delta_secs();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }
}
