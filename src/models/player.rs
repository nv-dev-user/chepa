use crate::models::living_entity::LivingEntityCombat;

use super::living_entity::LivingEntity;
use super::npc::NPC;

pub struct Player {
    living_entity: LivingEntity,
    coef_exp: f32,
}

impl Player {
    pub fn new(
        living_entity: LivingEntity, 
        coef_exp: f32
    ) -> Self {
        Player { 
            living_entity, 
            coef_exp 
        }
    }

    pub fn get_living_entity(&self) -> &LivingEntity {
        &self.living_entity
    }

    pub fn get_mut_living_entity(&mut self) -> &mut LivingEntity {
        &mut self.living_entity
    }

    pub fn get_coef_exp(&self) -> f32 {
        self.coef_exp
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