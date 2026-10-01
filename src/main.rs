use bevy::prelude::*;

mod combat;
mod player;
mod ui;
mod world;

use combat::CombatPlugin;
use player::PlayerPlugin;
use ui::UiPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "JDR - Exploration".into(),
                resolution: (1000, 700).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            WorldPlugin,
            PlayerPlugin,
            CombatPlugin,
            UiPlugin,
        ))
        .run();
}
