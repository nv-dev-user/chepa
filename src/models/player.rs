use super::armor::Armor;
use super::entity::Entity;
use super::zone::Zone;
use super::weapon::Weapon;

pub struct Player {
    entity: Entity,
    zone: Zone,
    armor: Vec<Armor>,
    weapon: Vec<Weapon>,
    xp: i32,
    level: i32,
    xp_max: i32
}

impl Player {
    pub fn new(entity: Entity, zone: Zone, armor: Vec<Armor>, weapon: Vec<Weapon>, xp: i32, level: i32, xp_max: i32) -> Self {
        Player { entity, zone, armor, weapon, xp, level, xp_max }
    }

    pub fn get_entity(&self) -> &Entity {
        &self.entity
    }

    pub fn get_zone(&self) -> &Zone {
        &self.zone
    }

    pub fn get_armor(&self) -> &Vec<Armor> {
        &self.armor
    }

    pub fn get_weapon(&self) -> &Vec<Weapon> {
        &self.weapon
    }

    pub fn get_xp(&self) -> i32 {
        self.xp
    }

    pub fn get_level(&self) -> i32 {
        self.level
    }

    pub fn get_xp_max(&self) -> i32 {
        self.xp_max
    }
}