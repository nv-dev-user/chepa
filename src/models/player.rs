use crate::models::living_entity::LivingEntityCombat;

use super::living_entity::LivingEntity;
use super::npc::NPC;

#[derive(Debug, serde::Deserialize)]
pub struct Player {
    living_entity: LivingEntity,
    xp_total: u64,
}

impl Player {
    const XP_BASE: f64 = 100.0;
    const XP_CURVE: f64 = 2.0;

    pub fn new(living_entity: LivingEntity, xp_total: u64) -> Self {
        Player {
            living_entity,
            xp_total,
        }
    }

    pub fn get_living_entity(&self) -> &LivingEntity {
        &self.living_entity
    }

    pub fn get_mut_living_entity(&mut self) -> &mut LivingEntity {
        &mut self.living_entity
    }

    pub fn get_xp_total(&self) -> u64 {
        self.xp_total
    }

    pub fn add_xp(&mut self, gained_xp: u32) {
        self.xp_total = self.xp_total.saturating_add(gained_xp as u64);
    }

    pub fn get_level(&self) -> u32 {
        let mut level = 0_u32;
        loop {
            let next_level = level.saturating_add(1);
            let required = Self::xp_required_for_level(next_level);
            if required > self.xp_total {
                break level;
            }
            level = next_level;
        }
    }

    pub fn xp_required_for_level(level: u32) -> u64 {
        if level == 0 {
            return 0;
        }

        (Self::XP_BASE * f64::powf(level as f64, Self::XP_CURVE)).round() as u64
    }

    pub fn xp_to_next_level(&self) -> u64 {
        let next_level = self.get_level().saturating_add(1);
        let next_required = Self::xp_required_for_level(next_level);
        next_required.saturating_sub(self.xp_total)
    }
  
    // TODO : mettre là où ça doit aller
    pub fn fight(&mut self, target: &mut NPC) {
        while self.is_alive() && target.is_alive() {
            self.attack(target.get_mut_living_entity());

            if !target.is_alive() {
                // TODO: add xp to player
                // TODO: add items to player
                break;
            }

            target.attack(self.get_mut_living_entity());

            if !self.is_alive() {
                // TODO: remove items from player
                // TODO: respawn
                break;
            }
        }
    }
}

impl LivingEntityCombat for Player {
    fn attack(&self, target: &mut LivingEntity) {
        self.living_entity.attack(target);
    }

    fn take_damage(&mut self, damage: u32) {
        self.living_entity.take_damage(damage);
    }

    fn is_alive(&self) -> bool {
        self.living_entity.is_alive()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::entity::Entity;
    use crate::models::living_entity::LivingEntity;
    use crate::models::zone::Zone;

    fn make_zone() -> Zone {
        Zone::new(
            Entity::new(1, "Plaine".to_string()),
            1,
            50,
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn make_living_entity(id: u32, name: &str) -> LivingEntity {
        LivingEntity::new(
            Entity::new(id, name.to_string()),
            100,
            12,
            10,
            8,
            50,
            None,
            None,
            make_zone(),
            make_zone(),
        )
    }

    #[test]
    fn xp_required_for_level_follows_square_curve() {
        assert_eq!(Player::xp_required_for_level(0), 0);
        assert_eq!(Player::xp_required_for_level(1), 100);
        assert_eq!(Player::xp_required_for_level(2), 400);
        assert_eq!(Player::xp_required_for_level(3), 900);
    }

    #[test]
    fn level_is_computed_from_xp_total() {
        let p0 = Player::new(make_living_entity(10, "P0"), 0);
        let p1 = Player::new(make_living_entity(11, "P1"), 100);
        let p2 = Player::new(make_living_entity(12, "P2"), 850);

        assert_eq!(p0.get_level(), 0);
        assert_eq!(p1.get_level(), 1);
        assert_eq!(p2.get_level(), 2);
    }

    #[test]
    fn add_xp_updates_total_and_level() {
        let mut player = Player::new(make_living_entity(20, "Hero"), 350);
        assert_eq!(player.get_level(), 1);

        player.add_xp(50);
        assert_eq!(player.get_xp_total(), 400);
        assert_eq!(player.get_level(), 2);
    }

    #[test]
    fn xp_to_next_level_is_correct() {
        let p = Player::new(make_living_entity(30, "Mage"), 450);
        // level 2 => next level threshold is 900
        assert_eq!(p.get_level(), 2);
        assert_eq!(p.xp_to_next_level(), 450);
    }
}