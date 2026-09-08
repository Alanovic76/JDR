use bevy::prelude::*;

use crate::player::{Health, Player};

use super::components::HealthText;

pub fn spawn_hud(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("PV: --/--"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                HealthText,
            ));
        });
}

pub fn update_health_text(
    player_query: Query<&Health, With<Player>>,
    mut text_query: Query<&mut Text, With<HealthText>>,
) {
    let Ok(health) = player_query.single() else {
        return;
    };
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    **text = format!("PV: {}/{}", health.current, health.max);
}
