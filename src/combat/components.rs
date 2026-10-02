use bevy::prelude::*;
use std::{fs, time::{SystemTime, UNIX_EPOCH}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MonsterType { Gobelin, Renard, Loup, Hobgobelin }
impl MonsterType {
    pub fn name(self)->&'static str { match self { Self::Gobelin=>"Gobelin",Self::Renard=>"Renard",Self::Loup=>"Loup",Self::Hobgobelin=>"Hobgobelin" } }
    pub fn stats(self)->(i32,i32,i32,i32){ match self { Self::Gobelin=>(8,11,2,4),Self::Renard=>(6,12,3,3),Self::Loup=>(10,13,4,5),Self::Hobgobelin=>(14,14,5,6) } }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Weapon { Sword, Hammer, SwordShield, Axe }
impl Weapon {
    pub fn name(self)->&'static str { match self { Self::Sword=>"Épée",Self::Hammer=>"Marteau",Self::SwordShield=>"Épée + bouclier",Self::Axe=>"Hache" } }
    pub fn damage_label(self)->&'static str { match self { Self::Sword=>"1D8",Self::Hammer=>"2D3",Self::SwordShield=>"1D6",Self::Axe=>"1D10" } }
    pub fn defense_bonus(self)->i32 { if self==Self::SwordShield {2}else{0} }
    pub fn roll_damage(self,rng:&mut DiceRng)->i32 { match self { Self::Sword=>rng.roll(8),Self::Hammer=>rng.roll(3)+rng.roll(3),Self::SwordShield=>rng.roll(6),Self::Axe=>rng.roll(10) } }
}

#[derive(Resource, Debug)]
pub struct Equipment { pub weapon: Option<Weapon> }
impl Default for Equipment { fn default()->Self { Self{weapon:None} } }

#[derive(Resource, Debug, Default)]
pub struct Score { pub monsters_killed:u32, pub gold:u32 }

#[derive(Clone,Debug,Default)] pub struct HighScoreEntry { pub name:String,pub monsters:u32,pub gold:u32 }
#[derive(Resource,Debug,Default)] pub struct HighScores { pub entries:Vec<HighScoreEntry> }
impl HighScores {
 const FILE:&'static str="jdr_highscores.txt";
 pub fn load()->Self { let mut out=Self::default(); if let Ok(s)=fs::read_to_string(Self::FILE){for l in s.lines(){let p:Vec<_>=l.split('|').collect();if p.len()==3{if let(Ok(m),Ok(g))=(p[1].parse(),p[2].parse()){out.entries.push(HighScoreEntry{name:p[0].to_string(),monsters:m,gold:g});}}}} out.sort();out }
 fn sort(&mut self){self.entries.sort_by(|a,b|(b.monsters,b.gold).cmp(&(a.monsters,a.gold)));self.entries.truncate(10);}
 pub fn update(&mut self,name:&str,score:&Score){if let Some(e)=self.entries.iter_mut().find(|e|e.name.eq_ignore_ascii_case(name)){e.monsters=e.monsters.max(score.monsters_killed);e.gold=e.gold.max(score.gold);}else{self.entries.push(HighScoreEntry{name:name.to_string(),monsters:score.monsters_killed,gold:score.gold});}self.sort();let data=self.entries.iter().map(|e|format!("{}|{}|{}",e.name,e.monsters,e.gold)).collect::<Vec<_>>().join("\n");let _=fs::write(Self::FILE,data+"\n");}
 pub fn display(&self)->String{if self.entries.is_empty(){return "Aucun score".into();}self.entries.iter().take(5).enumerate().map(|(i,e)|format!("{}. {}  M:{}  Or:{}",i+1,e.name,e.monsters,e.gold)).collect::<Vec<_>>().join("\n")}
}

#[derive(Resource, Debug, Default)]
pub struct TreasureState { pub pending:bool, pub message:String, pub grid:Option<IVec2> }

#[derive(Component, Clone, Copy, Debug)]
pub struct PlayerStats { pub for_:i32,pub int_:i32,pub con:i32,pub wis:i32,pub dex:i32,pub cha:i32 }
impl Default for PlayerStats { fn default()->Self { Self{for_:12,int_:10,con:13,wis:10,dex:12,cha:10} } }
impl PlayerStats { pub fn modifier(v:i32)->i32{(v-10)/2} pub fn for_mod(self)->i32{Self::modifier(self.for_)} pub fn dex_mod(self)->i32{Self::modifier(self.dex)} }

#[derive(Resource,Debug)]
pub struct CombatState { pub active:bool,pub rolling:bool,pub monster:Option<MonsterType>,pub monster_hp:i32,pub monster_max_hp:i32,pub player_hp:i32,pub player_max_hp:i32,pub last_roll:Option<i32>,pub last_message:String,pub game_over:bool,pub victory:bool,pub first_attack:bool,pub surprise:i8 }
impl Default for CombatState { fn default()->Self{Self{active:false,rolling:false,monster:None,monster_hp:0,monster_max_hp:0,player_hp:20,player_max_hp:20,last_roll:None,last_message:String::new(),game_over:false,victory:false,first_attack:false,surprise:0}} }
impl CombatState {
    pub fn begin_encounter(&mut self,monster:MonsterType,rng:&mut DiceRng){
        let (hp,_,_,_)=monster.stats(); let p=rng.roll(20); let m=rng.roll(20); self.surprise=if p>m{1}else if p<m{-1}else{0};
        self.active=true;self.rolling=true;self.monster=Some(monster);self.monster_hp=hp;self.monster_max_hp=hp;self.last_roll=None;self.game_over=false;self.victory=false;self.first_attack=true;
        let s=match self.surprise{1=>"Vous avez la surprise : AVANTAGE au premier jet.",-1=>"Le monstre vous surprend : DÉSAVANTAGE au premier jet.",_=>"Surprise égale : jet normal."};
        self.last_message=format!("Un {} surgit ! Surprise : vous {} / monstre {}. {}",monster.name(),p,m,s);
    }
}

#[derive(Resource,Debug)] pub struct DiceRng{state:u64}
impl Default for DiceRng{fn default()->Self{let seed=SystemTime::now().duration_since(UNIX_EPOCH).map(|d|d.as_nanos() as u64).unwrap_or(1);Self{state:seed.max(1)}}}
impl DiceRng{pub fn roll(&mut self,sides:i32)->i32{self.state=self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);((self.state>>32)%sides as u64) as i32+1}}
