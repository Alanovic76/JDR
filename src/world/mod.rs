use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

use bevy::prelude::*;

pub const TILE_SIZE: f32 = 32.0;
pub const MAP_WIDTH: usize = 25;
pub const MAP_HEIGHT: usize = 15;

const MAP: [&str; MAP_HEIGHT] = [
    "#########################",
    "#.......................#",
    "#...R......V............#",
    "#................R......#",
    "#..V....R...............#",
    "#.........V......R......#",
    "#...R........V..........#",
    "#P.....................E#",
    "#......R.........V......#",
    "#............R..........#",
    "#.V.................R...#",
    "#........V..............#",
    "#......R.........V......#",
    "#.......................#",
    "#########################",
];

#[derive(Resource, Clone, Default)]
pub struct Level {
    pub walls: HashSet<IVec2>,
    pub exit: Option<IVec2>,
    pub player_start: Option<IVec2>,
    pub monster_spots: HashSet<IVec2>,
    pub finished: bool,
}

#[derive(Component)]
pub struct Exit;

#[derive(Component)]
struct Tile;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Level>()
            .add_systems(PreStartup, setup_world);
    }
}

pub fn grid_to_world(grid: IVec2) -> Vec2 {
    let map_w = MAP_WIDTH as f32 * TILE_SIZE;
    let map_h = MAP_HEIGHT as f32 * TILE_SIZE;

    Vec2::new(
        grid.x as f32 * TILE_SIZE + TILE_SIZE / 2.0 - map_w / 2.0,
        map_h / 2.0 - (grid.y as f32 * TILE_SIZE + TILE_SIZE / 2.0),
    )
}

pub fn world_to_grid(position: Vec2) -> IVec2 {
    let map_w = MAP_WIDTH as f32 * TILE_SIZE;
    let map_h = MAP_HEIGHT as f32 * TILE_SIZE;

    IVec2::new(
        ((position.x + map_w / 2.0) / TILE_SIZE).floor() as i32,
        ((map_h / 2.0 - position.y) / TILE_SIZE).floor() as i32,
    )
}

pub fn is_blocked(level: &Level, world_position: Vec2) -> bool {
    let grid = world_to_grid(world_position);

    grid.x < 0
        || grid.x >= MAP_WIDTH as i32
        || grid.y < 0
        || grid.y >= MAP_HEIGHT as i32
        || level.walls.contains(&grid)
}

fn setup_world(mut commands: Commands, mut level: ResMut<Level>) {
    commands.spawn(Camera2d);

    let mut walkable_between = Vec::new();

    for (y, row) in MAP.iter().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            let grid = IVec2::new(x as i32, y as i32);
            let position = grid_to_world(grid);

            let floor_color = if (x + y) % 2 == 0 {
                Color::srgb(0.14, 0.20, 0.14)
            } else {
                Color::srgb(0.17, 0.23, 0.17)
            };

            commands.spawn((
                Sprite::from_color(floor_color, Vec2::splat(TILE_SIZE - 1.0)),
                Transform::from_xyz(position.x, position.y, 0.0),
                Tile,
            ));

            match cell {
                '#' => {
                    level.walls.insert(grid);
                    commands.spawn((
                        Sprite::from_color(
                            Color::srgb(0.32, 0.32, 0.35),
                            Vec2::splat(TILE_SIZE - 3.0),
                        ),
                        Transform::from_xyz(position.x, position.y, 1.0),
                    ));
                }
                'R' => {
                    level.walls.insert(grid);
                    commands.spawn((
                        Sprite::from_color(
                            Color::srgb(0.48, 0.48, 0.50),
                            Vec2::splat(TILE_SIZE - 5.0),
                        ),
                        Transform::from_xyz(position.x, position.y, 1.0),
                    ));
                }
                'V' => {
                    level.walls.insert(grid);
                    commands.spawn((
                        Sprite::from_color(
                            Color::srgb(0.12, 0.55, 0.18),
                            Vec2::splat(TILE_SIZE - 5.0),
                        ),
                        Transform::from_xyz(position.x, position.y, 1.0),
                    ));
                }
                'P' => {
                    level.player_start = Some(grid);
                    commands.spawn((
                        Sprite::from_color(
                            Color::srgb(0.20, 0.55, 0.95),
                            Vec2::splat(TILE_SIZE - 6.0),
                        ),
                        Transform::from_xyz(position.x, position.y, 2.0),
                    ));
                }
                'E' => {
                    level.exit = Some(grid);
                    commands.spawn((
                        Sprite::from_color(
                            Color::srgb(1.0, 0.78, 0.05),
                            Vec2::splat(TILE_SIZE - 6.0),
                        ),
                        Transform::from_xyz(position.x, position.y, 2.0),
                        Exit,
                    ));
                }
                '.' => {
                    if x > 2 && x < MAP_WIDTH - 2 && y > 1 && y < MAP_HEIGHT - 2 {
                        walkable_between.push(grid);
                    }
                }
                _ => {}
            }
        }
    }

    if !walkable_between.is_empty() {
        let mut state = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(1)
            .max(1);

        // Entre 4 et 6 rencontres, placées sur des cases praticables distinctes.
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let monster_count = 4 + ((state >> 32) % 3) as usize;

        for _ in 0..monster_count.min(walkable_between.len()) {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let index = ((state >> 32) as usize) % walkable_between.len();
            let grid = walkable_between.swap_remove(index);
            level.monster_spots.insert(grid);
        }

        // Rien n'est dessiné : les monstres restent invisibles pendant l'exploration.
    }
}
