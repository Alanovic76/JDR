use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode};

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
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((WorldPlugin, PlayerPlugin, CombatPlugin, UiPlugin))
        .run();
}
