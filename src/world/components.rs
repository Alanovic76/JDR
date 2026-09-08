use bevy::prelude::*;

pub const TILE_SIZE: f32 = 32.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TileKind {
    Grass,
    Tree,
    Water,
    DirtPath,
}

impl TileKind {
    pub fn color(self) -> Color {
        match self {
            TileKind::Grass => Color::srgb(0.2, 0.55, 0.2),
            TileKind::Tree => Color::srgb(0.1, 0.35, 0.1),
            TileKind::Water => Color::srgb(0.2, 0.4, 0.8),
            TileKind::DirtPath => Color::srgb(0.55, 0.4, 0.25),
        }
    }

    pub fn is_walkable(self) -> bool {
        !matches!(self, TileKind::Tree | TileKind::Water)
    }
}

#[derive(Component)]
pub struct Tile {
    pub kind: TileKind,
    pub grid_x: i32,
    pub grid_y: i32,
}
