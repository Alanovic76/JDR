use bevy::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MonsterType {
    Gobelin,
    Renard,
    Loup,
    Hobgobelin,
}

impl MonsterType {
    pub fn name(self) -> &'static str {
        match self {
            Self::Gobelin => "Gobelin",
            Self::Renard => "Renard",
            Self::Loup => "Loup",
            Self::Hobgobelin => "Hobgobelin",
        }
    }

    pub fn stats(self) -> (i32, i32, i32, i32) {
        match self {
            Self::Gobelin => (8, 11, 2, 4),
            Self::Renard => (6, 12, 3, 3),
            Self::Loup => (10, 13, 4, 5),
            Self::Hobgobelin => (14, 14, 5, 6),
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct PlayerStats {
    pub for_: i32,
    pub int_: i32,
    pub con: i32,
    pub wis: i32,
    pub dex: i32,
    pub cha: i32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            for_: 12,
            int_: 10,
            con: 13,
            wis: 10,
            dex: 12,
            cha: 10,
        }
    }
}

impl PlayerStats {
    pub fn modifier(value: i32) -> i32 {
        (value - 10) / 2
    }

    pub fn for_mod(self) -> i32 {
        Self::modifier(self.for_)
    }

    pub fn dex_mod(self) -> i32 {
        Self::modifier(self.dex)
    }

    pub fn con_mod(self) -> i32 {
        Self::modifier(self.con)
    }
}

#[derive(Resource, Debug)]
pub struct CombatState {
    pub active: bool,
    pub choosing_monster: bool,
    pub rolling: bool,
    pub monster: Option<MonsterType>,
    pub monster_hp: i32,
    pub monster_max_hp: i32,
    pub player_hp: i32,
    pub player_max_hp: i32,
    pub last_roll: Option<i32>,
    pub last_message: String,
    pub game_over: bool,
    pub victory: bool,
}

impl Default for CombatState {
    fn default() -> Self {
        Self {
            active: false,
            choosing_monster: false,
            rolling: false,
            monster: None,
            monster_hp: 0,
            monster_max_hp: 0,
            player_hp: 20,
            player_max_hp: 20,
            last_roll: None,
            last_message: String::new(),
            game_over: false,
            victory: false,
        }
    }
}

impl CombatState {
    pub fn begin_encounter(&mut self, monster: MonsterType) {
        let (hp, _defense, _attack, _damage) = monster.stats();
        self.active = true;
        self.choosing_monster = false;
        self.rolling = true;
        self.monster = Some(monster);
        self.monster_hp = hp;
        self.monster_max_hp = hp;
        self.last_roll = None;
        self.game_over = false;
        self.victory = false;
        self.last_message = format!(
            "Un {} surgit ! Appuyez sur ESPACE pour lancer le D20.",
            monster.name()
        );
    }
}

#[derive(Resource, Debug)]
pub struct DiceRng {
    state: u64,
}

impl Default for DiceRng {
    fn default() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x1234_5678_9ABC_DEF0);

        Self { state: seed.max(1) }
    }
}

impl DiceRng {
    pub fn roll(&mut self, sides: i32) -> i32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);

        ((self.state >> 32) % sides as u64) as i32 + 1
    }
}
