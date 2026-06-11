use super::armor::Armor;
use super::entity::Entity;
use super::weapon::Weapon;
use super::zone::Zone;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct LivingEntity {
    entity: Entity,
    base_hp: u32,
    base_attack: u32,
    base_speed: u32,
    base_parade: u32,
    base_xp: u32,
    weapon: Option<Weapon>,
    armor: Option<Armor>,
    zone: Option<Zone>,
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
        zone: Option<Zone>,
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

    pub fn get_zone(&self) -> Option<&Zone> {
        self.zone.as_ref()
    }

    pub fn get_come_from(&self) -> &Zone {
        &self.come_from
    }

    pub fn set_weapon(&mut self, weapon: &Weapon) {
        self.weapon = Some(weapon.clone());
    }

    pub fn set_armor(&mut self, armor: &Armor) {
        self.armor = Some(armor.clone());
    }

    pub fn set_zone(&mut self, zone: &Zone) {
        self.zone = Some(zone.clone());
    }

    pub fn set_come_from(&mut self, come_from: &Zone) {
        self.come_from = come_from.clone();
    }
}

pub trait LivingEntityCombat {
    fn attack(&self, target: &mut LivingEntity);
    fn take_damage(&mut self, damage: u32);
    fn is_alive(&self) -> bool;
}

impl LivingEntityCombat for LivingEntity {
    fn attack(&self, target: &mut LivingEntity) {
        let damage = if self.weapon.is_some() {
            self.base_attack * self.weapon.as_ref().unwrap().get_coef_modifier()
        } else {
            self.base_attack
        };

        if target.is_alive() {
            target.take_damage(damage);
        }
    }

    fn take_damage(&mut self, damage: u32) {
        let damage = if self.armor.is_some() {
            damage - self.base_parade * self.armor.as_ref().unwrap().get_coef_modifier()
        } else {
            damage
        };

        self.base_hp -= damage;
    }

    fn is_alive(&self) -> bool {
        self.base_hp > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let entity = Entity::new(1, "test".to_string());
        let weapon = Weapon::new(1, "test".to_string(), 1);
        let armor = Armor::new(1, "test".to_string(), 1);
        let zone_entity = Entity::new(1, "test".to_string());
        let zone = Zone::new(zone_entity, 1, 1, Some(1), Some(1), Some(1), Some(1), Some(1));
        let living_entity = LivingEntity::new(entity, 1, 1, 1, 1, 1, Some(weapon), Some(armor), Some(zone.clone()), zone.clone());
        assert_eq!(living_entity.get_entity().get_id(), 1);
        assert_eq!(living_entity.get_entity().get_name(), "test");
        assert_eq!(living_entity.get_base_hp(), 1);
        assert_eq!(living_entity.get_base_attack(), 1);
        assert_eq!(living_entity.get_base_speed(), 1);
        assert_eq!(living_entity.get_base_parade(), 1);
        assert_eq!(living_entity.get_base_xp(), 1);
        assert_eq!(living_entity.get_weapon().is_some(), true);
        assert_eq!(living_entity.get_armor().is_some(), true);
        assert_eq!(living_entity.get_zone().as_ref().unwrap().get_entity().get_id(), 1);
        assert_eq!(living_entity.get_zone().as_ref().unwrap().get_entity().get_name(), "test");
    }

    #[test]
    fn test_attack() {
        let entity = Entity::new(1, "test".to_string());
        let weapon = Weapon::new(1, "test".to_string(), 2);
        let armor = Armor::new(1, "test".to_string(), 1);
        let zone_entity = Entity::new(1, "test".to_string());
        let zone = Zone::new(zone_entity, 1, 1, Some(1), Some(1), Some(1), Some(1), Some(1));
        let living_entity = LivingEntity::new(entity, 1, 1, 1, 1, 1, Some(weapon), Some(armor), Some(zone.clone()), zone.clone());

        let target_entity = Entity::new(2, "target".to_string());
        let target_weapon = Weapon::new(2, "target".to_string(), 1);
        let target_armor = Armor::new(2, "target".to_string(), 1);
        let target_zone_entity = Entity::new(1, "test".to_string());
        let target_zone = Zone::new(target_zone_entity, 1, 1, Some(1), Some(1), Some(1), Some(1), Some(1));
        let mut target = LivingEntity::new(target_entity, 1, 1, 1, 1, 1, Some(target_weapon), Some(target_armor), Some(target_zone.clone()), target_zone.clone());

        living_entity.attack(&mut target);
        assert_eq!(target.get_base_hp(), 0);
    }

    #[test]
    fn test_take_damage() {
        let entity = Entity::new(1, "test".to_string());
        let weapon = Weapon::new(1, "test".to_string(), 1);
        let armor = Armor::new(1, "test".to_string(), 1);
        let zone_entity = Entity::new(1, "test".to_string());
        let zone = Zone::new(zone_entity, 1, 1, Some(1), Some(1), Some(1), Some(1), Some(1));
        let mut living_entity = LivingEntity::new(entity, 1, 1, 1, 1, 1, Some(weapon), Some(armor), Some(zone.clone()), zone.clone());
        living_entity.take_damage(2);
        assert_eq!(living_entity.get_base_hp(), 0);
    }

    #[test]
    fn test_is_alive() {
        let entity = Entity::new(1, "test".to_string());
        let weapon = Weapon::new(1, "test".to_string(), 1);
        let armor = Armor::new(1, "test".to_string(), 1);
        let zone_entity = Entity::new(1, "test".to_string());
        let zone = Zone::new(zone_entity, 1, 1, Some(1), Some(1), Some(1), Some(1), Some(1));
        let living_entity = LivingEntity::new(entity, 1, 1, 1, 1, 1, Some(weapon), Some(armor), Some(zone.clone()), zone.clone());
        assert_eq!(living_entity.is_alive(), true);
    } 
}
