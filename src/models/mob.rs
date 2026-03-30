use super::entity::Entity;
use super::weapon::Weapon;

pub struct Mob {
    entity: Entity,
    base_hp: u32,
    base_attack: u32,
    base_speed: u32,
    base_parade: u32,
    base_xp: u32,
    drop_rate: Vec<Weapon>,
    weapon: Weapon,
}

impl Mob {
    pub fn new(id: u32, name: String, base_hp: u32, base_attack: u32, base_speed: u32, base_parade: u32, base_xp: u32, drop_rate: Vec<Weapon>, weapon: Weapon) -> Self {
        Mob { entity: Entity::new(id, name), base_hp, base_attack, base_speed, base_parade, base_xp, drop_rate, weapon }
    }

    pub fn get_id(&self) -> u32 {
        self.entity.get_id()
    }

    pub fn get_name(&self) -> &str {
        self.entity.get_name()
    }

    pub fn get_base_hp(&self) -> u32 {
        self.base_hp
    }

    pub fn get_base_attack(&self) -> u32 {
        self.base_attack
    }

    pub fn get_base_speed(&self) -> u32 {
        self.base_speed
    }

    pub fn get_base_parade(&self) -> u32 {
        self.base_parade
    }
    
    pub fn get_base_xp(&self) -> u32 {
        self.base_xp
    }

    pub fn get_drop_rate(&self) -> &Vec<Weapon> {
        &self.drop_rate
    }
    
    pub fn get_weapon(&self) -> &Weapon {
        &self.weapon
    }
}