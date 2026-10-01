use bevy::prelude::*;

use crate::combat::{CombatState, PlayerStats};
use crate::player::Player;
use crate::world::Level;

#[derive(Component)]
struct StatusText;

#[derive(Component)]
struct StatsText;

#[derive(Component)]
struct CombatText;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(10),
            left: px(10),
            padding: UiRect::all(px(10)),
            flex_direction: FlexDirection::Column,
            row_gap: px(4),
            ..default()
        },
        BackgroundColor(Color::srgba(0.03, 0.05, 0.03, 0.88)),
        children![
            (
                Text::new("JDR — NIVEAU 01"),
                TextFont::from_font_size(22.0),
                TextColor(Color::WHITE),
            ),
            (
                Text::new("Déplacement : ZQSD / WASD / Flèches"),
                TextFont::from_font_size(15.0),
                TextColor(Color::srgb(0.75, 0.85, 0.75)),
            ),
            (
                Text::new("FOR 12  INT 10  CON 13  WIS 10  DEX 12  CHA 10"),
                TextFont::from_font_size(15.0),
                TextColor(Color::srgb(0.95, 0.90, 0.55)),
                StatsText,
            ),
            (
                Text::new("Exploration..."),
                TextFont::from_font_size(15.0),
                TextColor(Color::srgb(0.40, 1.0, 0.55)),
                StatusText,
            ),
        ],
    ));

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(220),
                right: px(220),
                top: px(150),
                bottom: px(150),
                padding: UiRect::all(px(25)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.04, 0.02, 0.96)),
            CombatPanel,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                TextFont::from_font_size(24.0),
                TextColor(Color::WHITE),
                CombatText,
            ));
        });
}

#[derive(Component)]
struct CombatPanel;

fn update_ui(
    combat: Res<CombatState>,
    level: Res<Level>,
    player_query: Query<(&PlayerStats, &Transform), With<Player>>,
    mut status_query: Query<&mut Text, (With<StatusText>, Without<CombatText>)>,
    mut combat_text_query: Query<&mut Text, (With<CombatText>, Without<StatusText>)>,
    mut panel_query: Query<&mut Node, With<CombatPanel>>,
) {
    let Ok((stats, player_transform)) = player_query.single() else {
        return;
    };

    let Ok(mut status) = status_query.single_mut() else {
        return;
    };

    let Ok(mut combat_text) = combat_text_query.single_mut() else {
        return;
    };

    let Ok(mut panel) = panel_query.single_mut() else {
        return;
    };

    if combat.active {
        panel.display = Display::Flex;

            let monster_name = combat.monster.map(|m| m.name()).unwrap_or("?");
            *combat_text = Text::new(format!(
                "COMBAT — {}\n\n\
                 Vos PV : {}/{}\n\
                 PV du monstre : {}/{}\n\n\
                 FOR {} (mod {:+})   DEX {} (mod {:+})\n\
                 CON {} (mod {:+})\n\n\
                 {}\n\n\
                 [ESPACE] lancer le D20",
                monster_name,
                combat.player_hp,
                combat.player_max_hp,
                combat.monster_hp,
                combat.monster_max_hp,
                stats.for_,
                stats.for_mod(),
                stats.dex,
                stats.dex_mod(),
                stats.con,
                stats.con_mod(),
                combat.last_message
            ));
    } else {
        panel.display = Display::None;

        if combat.victory {
            *status = Text::new("Victoire ! Continuez vers la sortie jaune.");
        } else if combat.game_over {
            *status = Text::new("Vous êtes vaincu. Fermez et relancez le jeu pour recommencer.");
        } else if level.finished {
            *status = Text::new("NIVEAU TERMINE ! 🎉");
        } else if let Some(exit) = level.exit {
            let player_grid = crate::world::world_to_grid(player_transform.translation.truncate());
            if player_grid == exit {
                *status = Text::new("NIVEAU TERMINE ! 🎉");
            } else {
                *status = Text::new("En exploration...");
            }
        } else {
            *status = Text::new("En exploration...");
        }
    }
}
