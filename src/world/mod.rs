use std::collections::HashSet;

use bevy::prelude::*;

pub const TILE_SIZE: f32 = 32.0;
pub const MAP_WIDTH: usize = 25;
pub const MAP_HEIGHT: usize = 15;

const MAP: [&str; MAP_HEIGHT] = [
    "#########################",
    "#P....#.................#",
    "#.##..#..#####..........#",
    "#.....#..#...#..........#",
    "#.....#..#...#..#####...#",
    "#........#...#..........#",
    "#####.###.###.#########.#",
    "#.....#.................#",
    "#.###.#.#############..#",
    "#...#.#..............#..#",
    "###.#.###########.##.#..#",
    "#...#.............#..#..#",
    "#.###############.#..#..#",
    "#.................#...E.#",
    "#########################",
];

#[derive(Resource, Clone, Default)]
pub struct Level {
    pub walls: HashSet<IVec2>,
    pub exit: Option<IVec2>,
    pub player_start: Option<IVec2>,
}

#[derive(Component)]
pub struct Exit;

#[derive(Component)]
pub struct Npc;

#[derive(Component)]
struct Tile;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Level>()
            .add_systems(Startup, setup_world);
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

    let x = ((position.x + map_w / 2.0) / TILE_SIZE).floor() as i32;
    let y = ((map_h / 2.0 - position.y) / TILE_SIZE).floor() as i32;

    IVec2::new(x, y)
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

    let mut npc_grid = None;

    for (y, row) in MAP.iter().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            let grid = IVec2::new(x as i32, y as i32);
            let position = grid_to_world(grid);

            let floor_color = if (x + y) % 2 == 0 {
                Color::srgb(0.16, 0.18, 0.20)
            } else {
                Color::srgb(0.19, 0.21, 0.23)
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
                            Color::srgb(0.08, 0.09, 0.11),
                            Vec2::splat(TILE_SIZE - 1.0),
                        ),
                        Transform::from_xyz(position.x, position.y, 1.0),
                    ));
                }
                'P' => {
                    level.player_start = Some(grid);
                    commands.spawn((
                        Sprite::from_color(
                            Color::srgb(0.20, 0.65, 1.0),
                            Vec2::splat(TILE_SIZE - 7.0),
                        ),
                        Transform::from_xyz(position.x, position.y, 2.0),
                    ));
                }
                'E' => {
                    level.exit = Some(grid);
                    commands.spawn((
                        Sprite::from_color(
                            Color::srgb(0.85, 0.65, 0.15),
                            Vec2::splat(TILE_SIZE - 6.0),
                        ),
                        Transform::from_xyz(position.x, position.y, 2.0),
                        Exit,
                    ));
                }
                '.' if npc_grid.is_none() && x > 10 && y > 5 => {
                    npc_grid = Some(grid);
                }
                _ => {}
            }
        }
    }

    if let Some(grid) = npc_grid {
        let position = grid_to_world(grid);

        commands.spawn((
            Sprite::from_color(
                Color::srgb(0.55, 0.25, 0.75),
                Vec2::splat(TILE_SIZE - 8.0),
            ),
            Transform::from_xyz(position.x, position.y, 2.0),
            Npc,
        ));
    }
}
