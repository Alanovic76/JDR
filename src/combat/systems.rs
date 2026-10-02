use bevy::prelude::*;

use crate::player::Player;
use super::{CombatState, DiceRng, PlayerStats};

pub fn combat_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut combat: ResMut<CombatState>,
    mut rng: ResMut<DiceRng>,
    player_query: Query<&PlayerStats, With<Player>>,
) {
    if !combat.active {
        return;
    }

    if combat.game_over || combat.victory {
        return;
    }

    if !combat.rolling || !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let Ok(stats) = player_query.single() else {
        return;
    };

    let Some(monster) = combat.monster else {
        return;
    };

    let (monster_max_hp, monster_defense, monster_attack, monster_damage) = monster.stats();

    let d20 = rng.roll(20);
    let attack_total = d20 + stats.for_mod();
    combat.last_roll = Some(d20);

    if d20 == 20 || attack_total >= monster_defense {
        let damage = rng.roll(6) + stats.for_mod().max(0);
        combat.monster_hp -= damage;

        if combat.monster_hp <= 0 {
            combat.monster_hp = 0;
            combat.rolling = false;
            combat.victory = true;
            combat.active = false;
            combat.last_message = format!(
                "Victoire ! D20 = {d20}, attaque = {attack_total}, {damage} dégâts. Le {} est vaincu.",
                monster.name()
            );
            return;
        }

        combat.last_message = format!(
            "Touché ! D20 = {d20}, attaque = {attack_total}, {damage} dégâts. Le {} a encore {}/{} PV.",
            monster.name(),
            combat.monster_hp,
            monster_max_hp
        );
    } else {
        combat.last_message = format!(
            "Raté ! D20 = {d20}, attaque = {attack_total} contre Défense {}.",
            monster_defense
        );
    }

    let monster_roll = rng.roll(20);
    let player_defense = 10 + stats.dex_mod();

    if monster_roll >= player_defense {
        let damage = rng.roll(monster_damage.max(1)) + (monster_attack / 2);
        combat.player_hp -= damage;

        if combat.player_hp <= 0 {
            combat.player_hp = 0;
            combat.game_over = true;
            combat.rolling = false;
            combat.last_message.push_str(&format!(
                " Le {} vous touche avec un D20 = {} et inflige {} dégâts. Vous êtes vaincu.",
                monster.name(),
                monster_roll,
                damage
            ));
        } else {
            let current_hp = combat.player_hp;
            let max_hp = combat.player_max_hp;
            combat.last_message.push_str(&format!(
                " Le {} vous touche (D20 = {}) : -{} PV. Il vous reste {}/{} PV.",
                monster.name(),
                monster_roll,
                damage,
                current_hp,
                max_hp
            ));
        }
    } else {
        combat.last_message.push_str(&format!(
            " Le {} rate son attaque (D20 = {}).",
            monster.name(),
            monster_roll
        ));
    }
}
