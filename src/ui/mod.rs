use bevy::app::AppExit;
use bevy::prelude::*;

use crate::combat::{CombatState, PlayerStats};
use crate::player::Player;
use crate::world::Level;

#[derive(Component)] struct StatusText;
#[derive(Component)] struct PlayerCardText;
#[derive(Component)] struct MonsterCardText;
#[derive(Component)] struct EndPanel;
#[derive(Component)] struct EndText;

#[derive(Resource, Default)] pub struct RestartRequest(pub bool);

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RestartRequest>()
            .add_systems(Startup, setup_ui)
            .add_systems(Update, (update_ui, end_controls));
    }
}

fn card_node(left: Option<f32>, right: Option<f32>) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: left.map(px).unwrap_or(Val::Auto),
        right: right.map(px).unwrap_or(Val::Auto),
        top: px(24),
        width: px(270),
        min_height: px(250),
        padding: UiRect::all(px(18)),
        border: UiRect::all(px(3)),
        flex_direction: FlexDirection::Column,
        row_gap: px(8),
        ..default()
    }
}

fn setup_ui(mut commands: Commands) {
    // Carte du personnage à gauche.
    commands.spawn((
        card_node(Some(24.0), None),
        BackgroundColor(Color::srgba(0.035, 0.055, 0.045, 0.94)),
        BorderColor::all(Color::srgb(0.38, 0.72, 0.46)),
        children![
            (Text::new("PERSONNAGE"), TextFont::from_font_size(25.0), TextColor(Color::WHITE)),
            (Text::new(""), TextFont::from_font_size(18.0), TextColor(Color::srgb(0.88, 0.95, 0.88)), PlayerCardText),
        ],
    ));

    // Carte du monstre à droite. Elle reste visible même hors combat.
    commands.spawn((
        card_node(None, Some(24.0)),
        BackgroundColor(Color::srgba(0.065, 0.035, 0.035, 0.94)),
        BorderColor::all(Color::srgb(0.75, 0.32, 0.28)),
        children![
            (Text::new("MONSTRE"), TextFont::from_font_size(25.0), TextColor(Color::WHITE)),
            (Text::new(""), TextFont::from_font_size(18.0), TextColor(Color::srgb(0.96, 0.88, 0.86)), MonsterCardText),
        ],
    ));

    // Bandeau d'état discret en bas, pour ne pas recouvrir la carte.
    commands.spawn((
        Node { position_type: PositionType::Absolute, left:px(320), right:px(320), bottom:px(18), padding:UiRect::axes(px(14), px(9)), justify_content:JustifyContent::Center, ..default() },
        BackgroundColor(Color::srgba(0.02,0.025,0.02,0.90)),
        children![(Text::new("Exploration..."), TextFont::from_font_size(17.0), TextColor(Color::srgb(0.72,0.92,0.72)), StatusText)],
    ));

    commands.spawn((
        Node { position_type:PositionType::Absolute, left:px(330), right:px(330), top:px(150), bottom:px(150), padding:UiRect::all(px(25)), align_items:AlignItems::Center, justify_content:JustifyContent::Center, display:Display::None, ..default() },
        BackgroundColor(Color::srgba(0.02,0.02,0.02,0.97)),
        children![(Text::new(""), TextFont::from_font_size(30.0), TextColor(Color::WHITE), EndText)],
        EndPanel,
    ));
}

fn update_ui(
    combat: Res<CombatState>, level: Res<Level>, player_query: Query<&PlayerStats, With<Player>>,
    mut player_card: Query<&mut Text,(With<PlayerCardText>,Without<MonsterCardText>,Without<StatusText>,Without<EndText>)>,
    mut monster_card: Query<&mut Text,(With<MonsterCardText>,Without<PlayerCardText>,Without<StatusText>,Without<EndText>)>,
    mut status_q: Query<&mut Text,(With<StatusText>,Without<PlayerCardText>,Without<MonsterCardText>,Without<EndText>)>,
    mut end_panel: Query<&mut Node,With<EndPanel>>,
    mut end_text: Query<&mut Text,(With<EndText>,Without<PlayerCardText>,Without<MonsterCardText>,Without<StatusText>)>,
) {
    let Ok(stats)=player_query.single() else{return;};

    if let Ok(mut t)=player_card.single_mut(){
        *t=Text::new(format!(
            "PV : {}/{}\n\nFOR : {}\nINT : {}\nCON : {}\nWIS : {}\nDEX : {}\nCHA : {}",
            combat.player_hp,combat.player_max_hp,stats.for_,stats.int_,stats.con,stats.wis,stats.dex,stats.cha
        ));
    }

    if let Ok(mut t)=monster_card.single_mut(){
        if let Some(m)=combat.monster {
            let (hp,def,atk,dmg)=m.stats();
            *t=Text::new(format!(
                "{}\n\nPV : {}/{}\nDéfense : {}\nAttaque : {}\nDégâts : D{}\n\n{}{}",
                m.name(), combat.monster_hp, hp, def, atk, dmg,
                combat.last_message,
                if combat.active {"\n\n[ESPACE] Lancer le D20"} else {""}
            ));
        } else {
            *t=Text::new("Aucun adversaire\n\nExplorez la carte.\nLes monstres sont invisibles jusqu'à la rencontre.");
        }
    }

    let ended=level.finished||combat.game_over;
    if let Ok(mut p)=end_panel.single_mut(){p.display=if ended{Display::Flex}else{Display::None};}
    if ended {
        if let Ok(mut t)=end_text.single_mut(){
            let reason=if combat.game_over{"Vous avez été vaincu."}else{"Vous avez atteint la sortie."};
            *t=Text::new(format!(
                "FIN\n\n{}\n\nPV : {}/{}\nFOR {}  INT {}  CON {}\nWIS {}  DEX {}  CHA {}\n\n[ENTRÉE] Nouvelle partie\n[ÉCHAP] Quitter",
                reason,combat.player_hp,combat.player_max_hp,stats.for_,stats.int_,stats.con,stats.wis,stats.dex,stats.cha
            ));
        }
    }

    if let Ok(mut s)=status_q.single_mut(){
        *s=Text::new(if combat.active {
            "COMBAT EN COURS - ESPACE pour lancer le D20"
        } else if ended {
            "Partie terminée."
        } else if combat.victory {
            "Victoire ! Continuez vers la sortie."
        } else {
            "Déplacement : ZQSD / WASD / Flèches"
        });
    }
}

fn end_controls(keyboard:Res<ButtonInput<KeyCode>>, level:Res<Level>, combat:Res<CombatState>, mut restart:ResMut<RestartRequest>, mut exit:MessageWriter<AppExit>){
    if !(level.finished||combat.game_over){return;}
    if keyboard.just_pressed(KeyCode::Enter){restart.0=true;}
    if keyboard.just_pressed(KeyCode::Escape){exit.write(AppExit::Success);}
}
