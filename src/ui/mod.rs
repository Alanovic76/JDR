use bevy::prelude::*;

pub struct UiPlugin;

#[derive(Component)]
struct StatusText;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_status);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            padding: UiRect::all(px(10)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.03, 0.04, 0.05, 0.85)),
        children![
            (
                Text::new("JDR — NIVEAU 01"),
                TextFont {
                    font_size: FontSize::Px(22.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                Text::new("Déplacement : ZQSD / WASD / Flèches"),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(Color::srgb(0.75, 0.80, 0.85)),
            ),
            (
                Text::new("Objectif : atteindre la sortie dorée."),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.85, 0.35)),
            ),
            (
                Text::new("En exploration..."),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(Color::srgb(0.40, 1.0, 0.55)),
                StatusText,
            ),
        ],
    ));
}

fn update_status(
    mut status_query: Query<&mut Text, With<StatusText>>,
    player_query: Query<&Transform, With<crate::player::Player>>,
    level: Res<crate::world::Level>,
) {
    let Ok(mut status) = status_query.single_mut() else {
        return;
    };

    let Ok(player) = player_query.single() else {
        return;
    };

    let Some(exit) = level.exit else {
        return;
    };

    let player_grid = crate::world::world_to_grid(player.translation.truncate());

    if player_grid == exit {
        *status = Text::new("NIVEAU TERMINE ! 🎉");
    } else {
        *status = Text::new("En exploration...");
    }
}
