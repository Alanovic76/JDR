use bevy::prelude::*;
use bevy::app::AppExit;

use crate::combat::{CombatState, PlayerStats};
use crate::player::Player;
use crate::world::Level;

#[derive(Component)] struct StatusText;
#[derive(Component)] struct StatsText;
#[derive(Component)] struct CombatText;
#[derive(Component)] struct CombatPanel;
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

fn setup_ui(mut commands: Commands) {
    commands.spawn((Node { position_type: PositionType::Absolute, top:px(10), left:px(10), padding:UiRect::all(px(10)), flex_direction:FlexDirection::Column, row_gap:px(4), ..default() }, BackgroundColor(Color::srgba(0.03,0.05,0.03,0.90)), children![
        (Text::new("JDR - NIVEAU 01"), TextFont::from_font_size(22.0), TextColor(Color::WHITE)),
        (Text::new("Déplacement : ZQSD / WASD / Flèches"), TextFont::from_font_size(15.0), TextColor(Color::srgb(0.75,0.85,0.75))),
        (Text::new(""), TextFont::from_font_size(15.0), TextColor(Color::srgb(0.95,0.90,0.55)), StatsText),
        (Text::new("Exploration..."), TextFont::from_font_size(15.0), TextColor(Color::srgb(0.40,1.0,0.55)), StatusText),
    ]));

    commands.spawn((Node { position_type:PositionType::Absolute, right:px(10), top:px(10), width:px(255), padding:UiRect::all(px(12)), flex_direction:FlexDirection::Column, ..default() }, BackgroundColor(Color::srgba(0.05,0.05,0.08,0.92)), children![
        (Text::new(""), TextFont::from_font_size(16.0), TextColor(Color::WHITE), CombatText)
    ], CombatPanel));

    commands.spawn((Node { position_type:PositionType::Absolute, left:px(180), right:px(180), top:px(170), bottom:px(170), padding:UiRect::all(px(25)), align_items:AlignItems::Center, justify_content:JustifyContent::Center, display:Display::None, ..default() }, BackgroundColor(Color::srgba(0.02,0.02,0.02,0.97)), children![
        (Text::new(""), TextFont::from_font_size(28.0), TextColor(Color::WHITE), EndText)
    ], EndPanel));
}

fn update_ui(
    combat: Res<CombatState>, level: Res<Level>, player_query: Query<&PlayerStats, With<Player>>,
    mut stats_q: Query<&mut Text,(With<StatsText>,Without<StatusText>,Without<CombatText>,Without<EndText>)>,
    mut status_q: Query<&mut Text,(With<StatusText>,Without<StatsText>,Without<CombatText>,Without<EndText>)>,
    mut combat_q: Query<&mut Text,(With<CombatText>,Without<StatsText>,Without<StatusText>,Without<EndText>)>,
    mut combat_panel: Query<&mut Node,(With<CombatPanel>,Without<EndPanel>)>,
    mut end_panel: Query<&mut Node,(With<EndPanel>,Without<CombatPanel>)>,
    mut end_text: Query<&mut Text,(With<EndText>,Without<StatsText>,Without<StatusText>,Without<CombatText>)>,
) {
    let Ok(stats)=player_query.single() else{return;};
    if let Ok(mut t)=stats_q.single_mut(){ *t=Text::new(format!("ÉTAT DU PERSONNAGE\nPV : {}/{}\nFOR {}   INT {}   CON {}\nWIS {}   DEX {}   CHA {}",combat.player_hp,combat.player_max_hp,stats.for_,stats.int_,stats.con,stats.wis,stats.dex,stats.cha)); }
    if let Ok(mut p)=combat_panel.single_mut(){p.display=if combat.active{Display::Flex}else{Display::None};}
    if combat.active {
        if let (Some(m),Ok(mut t))=(combat.monster,combat_q.single_mut()) { let (hp,def,atk,dmg)=m.stats(); *t=Text::new(format!("ÉTAT DU MONSTRE\n{}\nPV : {}/{}\nDéfense : {}\nAttaque : {}\nDégâts : D{}\n\n{}\n\n[ESPACE] lancer le D20",m.name(),combat.monster_hp,hp,def,atk,dmg,combat.last_message)); }
    }
    let ended=level.finished||combat.game_over;
    if let Ok(mut p)=end_panel.single_mut(){p.display=if ended{Display::Flex}else{Display::None};}
    if ended { if let Ok(mut t)=end_text.single_mut(){ let reason=if combat.game_over{"Vous avez été vaincu."}else{"Vous avez atteint la sortie."}; *t=Text::new(format!("FIN\n\n{}\n\nPV : {}/{}\nFOR {}  INT {}  CON {}\nWIS {}  DEX {}  CHA {}\n\n[ENTRÉE] Nouvelle partie\n[ÉCHAP] Quitter",reason,combat.player_hp,combat.player_max_hp,stats.for_,stats.int_,stats.con,stats.wis,stats.dex,stats.cha)); }}
    if let Ok(mut s)=status_q.single_mut(){ *s=Text::new(if combat.active{"Combat en cours..."}else if ended{"Partie terminée."}else if combat.victory{"Victoire ! Continuez vers la sortie."}else{"En exploration..."}); }
}

fn end_controls(keyboard:Res<ButtonInput<KeyCode>>, level:Res<Level>, combat:Res<CombatState>, mut restart:ResMut<RestartRequest>, mut exit:MessageWriter<AppExit>){
    if !(level.finished||combat.game_over){return;}
    if keyboard.just_pressed(KeyCode::Enter){restart.0=true;}
    if keyboard.just_pressed(KeyCode::Escape){exit.write(AppExit::Success);}
}
