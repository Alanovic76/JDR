use bevy::prelude::*;

use super::components::{Tile, TileKind, TILE_SIZE};

const MAP_WIDTH: i32 = 20;
const MAP_HEIGHT: i32 = 15;

/// Génère une carte de départ minimaliste : de l'herbe partout, une bordure
/// d'arbres, et un lac au centre. À remplacer plus tard par un chargement
/// de carte depuis assets/maps/.
pub fn spawn_world(mut commands: Commands) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let kind = tile_kind_for(x, y);

            let world_x = (x as f32 - MAP_WIDTH as f32 / 2.0) * TILE_SIZE;
            let world_y = (y as f32 - MAP_HEIGHT as f32 / 2.0) * TILE_SIZE;

            commands.spawn((
                Tile { kind, grid_x: x, grid_y: y },
                Sprite::from_color(kind.color(), Vec2::splat(TILE_SIZE)),
                Transform::from_xyz(world_x, world_y, 0.0),
            ));
        }
    }
}

fn tile_kind_for(x: i32, y: i32) -> TileKind {
    let is_border = x == 0 || y == 0 || x == MAP_WIDTH - 1 || y == MAP_HEIGHT - 1;
    if is_border {
        return TileKind::Tree;
    }

    let center_x = MAP_WIDTH / 2;
    let center_y = MAP_HEIGHT / 2;
    if (x - center_x).abs() <= 1 && (y - center_y).abs() <= 1 {
        return TileKind::Water;
    }

    if y == center_y {
        return TileKind::DirtPath;
    }

    TileKind::Grass
}
