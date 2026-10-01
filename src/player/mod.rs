use bevy::prelude::*;

use crate::combat::{CombatState, DiceRng, MonsterType, PlayerStats};
use crate::world::{grid_to_world, is_blocked, world_to_grid, Level};

#[derive(Component)]
pub struct Player;

const PLAYER_SIZE: f32 = 22.0;
const PLAYER_SPEED: f32 = 170.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, check_exit, check_monster));
    }
}

fn spawn_player(
    mut commands: Commands,
    level: Res<Level>,
) {
    let start = level.player_start.unwrap_or(IVec2::new(1, 7));
    let position = grid_to_world(start);

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.25, 0.70, 1.0),
            Vec2::splat(PLAYER_SIZE),
        ),
        Transform::from_xyz(position.x, position.y, 5.0),
        Player,
        PlayerStats::default(),
    ));
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    level: Res<Level>,
    combat: Res<CombatState>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if combat.active || level.finished {
        return;
    }

    let Ok(mut transform) = player_query.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::KeyZ) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction == Vec2::ZERO {
        return;
    }

    let direction = direction.normalize();
    let delta = direction * PLAYER_SPEED * time.delta_secs();
    let current = transform.translation.truncate();

    let next_x = current + Vec2::new(delta.x, 0.0);
    if can_player_move(level.as_ref(), next_x) {
        transform.translation.x = next_x.x;
    }

    let current_after_x = transform.translation.truncate();
    let next_y = current_after_x + Vec2::new(0.0, delta.y);
    if can_player_move(level.as_ref(), next_y) {
        transform.translation.y = next_y.y;
    }
}

fn can_player_move(level: &Level, position: Vec2) -> bool {
    let half = PLAYER_SIZE / 2.0 - 2.0;

    !is_blocked(level, position + Vec2::new(-half, -half))
        && !is_blocked(level, position + Vec2::new(half, -half))
        && !is_blocked(level, position + Vec2::new(-half, half))
        && !is_blocked(level, position + Vec2::new(half, half))
}

fn check_exit(
    mut level: ResMut<Level>,
    combat: Res<CombatState>,
    player_query: Query<&Transform, With<Player>>,
    mut exit_query: Query<&mut Sprite, With<crate::world::Exit>>,
) {
    if combat.active || level.finished {
        return;
    }

    let Ok(player) = player_query.single() else {
        return;
    };

    let Some(exit) = level.exit else {
        return;
    };

    let player_grid = world_to_grid(player.translation.truncate());

    for mut sprite in &mut exit_query {
        if player_grid == exit {
            level.finished = true;
            sprite.color = Color::srgb(0.30, 1.0, 0.35);
        } else if !level.finished {
            sprite.color = Color::srgb(1.0, 0.78, 0.05);
        }
    }
}

fn check_monster(
    mut level: ResMut<Level>,
    mut combat: ResMut<CombatState>,
    mut rng: ResMut<DiceRng>,
    player_query: Query<&Transform, With<Player>>,
) {
    if combat.active || combat.game_over {
        return;
    }

    let Ok(player) = player_query.single() else {
        return;
    };

    let player_grid = world_to_grid(player.translation.truncate());

    if level.monster_spots.remove(&player_grid) {
        let monster = match rng.roll(4) {
            1 => MonsterType::Gobelin,
            2 => MonsterType::Renard,
            3 => MonsterType::Loup,
            _ => MonsterType::Hobgobelin,
        };
        combat.begin_encounter(monster);
    }
}
