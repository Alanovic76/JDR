use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

use bevy::prelude::*;

use crate::player::Player;

pub const TILE_SIZE: f32 = 32.0;
pub const MAP_WIDTH: usize = 25;
pub const MAP_HEIGHT: usize = 15;

const OBSTACLE_COUNT: usize = 58;
const FOG_RADIUS: i32 = 1;

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

#[derive(Component)]
struct FogTile {
    grid: IVec2,
}

#[derive(Clone, Copy)]
enum ObstacleKind {
    Rock,
    Vegetation,
}

struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0xA17E_2026_0000_0001);
        Self { state: seed.max(1) }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        (self.state >> 32) as u32
    }

    fn range_i32(&mut self, min: i32, max_exclusive: i32) -> i32 {
        min + (self.next_u32() % (max_exclusive - min) as u32) as i32
    }

    fn range_usize(&mut self, max_exclusive: usize) -> usize {
        (self.next_u32() as usize) % max_exclusive
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Level>()
            .add_systems(PreStartup, setup_world)
            .add_systems(Update, reveal_fog);
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

    let mut rng = SimpleRng::new();

    // Départ à gauche et sortie à droite, avec une hauteur différente à chaque partie.
    let start = IVec2::new(1, rng.range_i32(2, MAP_HEIGHT as i32 - 2));
    let exit = IVec2::new(
        MAP_WIDTH as i32 - 2,
        rng.range_i32(2, MAP_HEIGHT as i32 - 2),
    );
    level.player_start = Some(start);
    level.exit = Some(exit);

    // Bordure bloquante.
    for y in 0..MAP_HEIGHT as i32 {
        for x in 0..MAP_WIDTH as i32 {
            if x == 0 || y == 0 || x == MAP_WIDTH as i32 - 1 || y == MAP_HEIGHT as i32 - 1 {
                level.walls.insert(IVec2::new(x, y));
            }
        }
    }

    // Ce couloir protégé garantit toujours un chemin entre départ et sortie.
    let mut protected_path = HashSet::new();
    for x in start.x..=exit.x {
        protected_path.insert(IVec2::new(x, start.y));
    }
    let min_y = start.y.min(exit.y);
    let max_y = start.y.max(exit.y);
    for y in min_y..=max_y {
        protected_path.insert(IVec2::new(exit.x, y));
    }

    // Rochers et végétation aléatoires, sans couper le chemin garanti.
    let mut obstacles = Vec::new();
    let mut attempts = 0;
    while obstacles.len() < OBSTACLE_COUNT && attempts < 5000 {
        attempts += 1;
        let grid = IVec2::new(
            rng.range_i32(1, MAP_WIDTH as i32 - 1),
            rng.range_i32(1, MAP_HEIGHT as i32 - 1),
        );

        if grid == start
            || grid == exit
            || protected_path.contains(&grid)
            || level.walls.contains(&grid)
        {
            continue;
        }

        level.walls.insert(grid);
        let kind = if rng.next_u32() % 2 == 0 {
            ObstacleKind::Rock
        } else {
            ObstacleKind::Vegetation
        };
        obstacles.push((grid, kind));
    }

    // Sol sur toute la carte.
    for y in 0..MAP_HEIGHT as i32 {
        for x in 0..MAP_WIDTH as i32 {
            let grid = IVec2::new(x, y);
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
        }
    }

    // Bordure grise.
    for grid in level.walls.iter().copied() {
        if grid.x == 0
            || grid.y == 0
            || grid.x == MAP_WIDTH as i32 - 1
            || grid.y == MAP_HEIGHT as i32 - 1
        {
            let position = grid_to_world(grid);
            commands.spawn((
                Sprite::from_color(Color::srgb(0.32, 0.32, 0.35), Vec2::splat(TILE_SIZE - 3.0)),
                Transform::from_xyz(position.x, position.y, 1.0),
            ));
        }
    }

    // Obstacles intérieurs.
    for (grid, kind) in obstacles {
        let position = grid_to_world(grid);
        let color = match kind {
            ObstacleKind::Rock => Color::srgb(0.48, 0.48, 0.50),
            ObstacleKind::Vegetation => Color::srgb(0.12, 0.55, 0.18),
        };
        commands.spawn((
            Sprite::from_color(color, Vec2::splat(TILE_SIZE - 5.0)),
            Transform::from_xyz(position.x, position.y, 1.0),
        ));
    }

    // Marqueur de départ.
    let start_pos = grid_to_world(start);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.20, 0.55, 0.95), Vec2::splat(TILE_SIZE - 6.0)),
        Transform::from_xyz(start_pos.x, start_pos.y, 2.0),
    ));

    // Sortie jaune.
    let exit_pos = grid_to_world(exit);
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.78, 0.05), Vec2::splat(TILE_SIZE - 6.0)),
        Transform::from_xyz(exit_pos.x, exit_pos.y, 2.0),
        Exit,
    ));

    // 4 à 6 monstres invisibles sur des cases libres et distinctes.
    let mut free_tiles = Vec::new();
    for y in 1..MAP_HEIGHT as i32 - 1 {
        for x in 1..MAP_WIDTH as i32 - 1 {
            let grid = IVec2::new(x, y);
            if grid != start && grid != exit && !level.walls.contains(&grid) {
                free_tiles.push(grid);
            }
        }
    }

    let monster_count = 4 + rng.range_usize(3);
    for _ in 0..monster_count.min(free_tiles.len()) {
        let index = rng.range_usize(free_tiles.len());
        let grid = free_tiles.swap_remove(index);
        level.monster_spots.insert(grid);
    }

    // Brouillard : une tuile noire au-dessus de chaque case. Elles seront retirées
    // définitivement à mesure que le joueur explore la carte.
    for y in 0..MAP_HEIGHT as i32 {
        for x in 0..MAP_WIDTH as i32 {
            let grid = IVec2::new(x, y);
            let position = grid_to_world(grid);
            let initially_visible = (grid.x - start.x).abs() > FOG_RADIUS
                || (grid.y - start.y).abs() > FOG_RADIUS;

            commands.spawn((
                Sprite::from_color(Color::srgb(0.015, 0.018, 0.02), Vec2::splat(TILE_SIZE)),
                Transform::from_xyz(position.x, position.y, 4.0),
                FogTile { grid },
                if initially_visible { Visibility::Visible } else { Visibility::Hidden },
            ));
        }
    }
}

fn reveal_fog(
    player_query: Query<&Transform, With<Player>>,
    mut fog_query: Query<(&FogTile, &mut Visibility)>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };

    let player_grid = world_to_grid(player.translation.truncate());

    for (fog, mut visibility) in &mut fog_query {
        if (fog.grid.x - player_grid.x).abs() <= FOG_RADIUS
            && (fog.grid.y - player_grid.y).abs() <= FOG_RADIUS
        {
            *visibility = Visibility::Hidden;
        }
    }
}
