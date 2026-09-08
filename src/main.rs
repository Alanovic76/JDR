use bevy::prelude::*;

mod player;
mod world;
mod combat;
mod enemies;
mod ui;

use player::PlayerPlugin;
use world::WorldPlugin;
use combat::CombatPlugin;
use enemies::EnemiesPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "JDR".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            WorldPlugin,
            PlayerPlugin,
            EnemiesPlugin,
            CombatPlugin,
            UiPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
