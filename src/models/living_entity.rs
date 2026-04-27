use super::armor::Armor;
use super::entity::Entity;
use super::weapon::Weapon;
use super::zone::Zone;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct LivingEntity {
    entity: Entity,
    base_hp: u32,
    base_attack: u32,
    base_speed: u32,
    base_parade: u32,
    base_xp: u32,
    weapon: Option<Weapon>,
    armor: Option<Armor>,
    zone: Zone,
    come_from: Zone,
}

impl LivingEntity {
    pub fn new(
        entity: Entity,
        base_hp: u32,
        base_attack: u32,
        base_speed: u32,
        base_parade: u32,
        base_xp: u32,
        weapon: Option<Weapon>,
        armor: Option<Armor>,
        zone: Zone,
        come_from: Zone,
    ) -> Self {
        Self {
            entity,
            base_hp,
            base_attack,
            base_speed,
            base_parade,
            base_xp,
            weapon,
            armor,
            zone,
            come_from,
        }
    }

    pub fn get_entity(&self) -> &Entity {
        &self.entity
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

    pub fn get_weapon(&self) -> Option<&Weapon> {
        self.weapon.as_ref()
    }

    pub fn get_armor(&self) -> Option<&Armor> {
        self.armor.as_ref()
    }

    pub fn get_zone(&self) -> &Zone {
        &self.zone
    }

    pub fn get_come_from(&self) -> &Zone {
        &self.come_from
    }

    pub fn set_weapon(&mut self, weapon: Weapon) {
        self.weapon = Some(weapon);
    }

    pub fn set_armor(&mut self, armor: Armor) {
        self.armor = Some(armor);
    }

    pub fn set_zone(&mut self, zone: Zone) {
        self.zone = zone;
    }

    pub fn set_come_from(&mut self, come_from: Zone) {
        self.come_from = come_from;
    }
}