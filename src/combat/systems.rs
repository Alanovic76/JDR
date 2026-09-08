use bevy::prelude::*;

use crate::player::Player;

use super::components::AttackEvent;

pub fn attack_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
    mut attack_events: EventWriter<AttackEvent>,
) {
    if !keyboard.just_pressed(KeyCode::KeyA) {
        return;
    }

    let Ok(player_entity) = player_query.single() else {
        return;
    };

    attack_events.write(AttackEvent { attacker: player_entity });
}
