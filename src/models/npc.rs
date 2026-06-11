use crate::models::living_entity::LivingEntityCombat;

use super::item::Item;
use std::collections::HashMap;
use super::living_entity::LivingEntity;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct NPC {
    living_entity: LivingEntity,
    dialogs: Vec<String>,
    dialog_index: usize,
    droppable_items_percentage : HashMap<Item, f32>,
}

impl NPC {
    pub fn new(
        living_entity: LivingEntity,
        dialogs: Vec<String>,
        droppable_items_percentage: HashMap<Item, f32>
    ) -> Self {
        NPC {
            living_entity,
            dialogs,
            dialog_index: 0,
            droppable_items_percentage }
    }

    pub fn get_living_entity(&self) -> &LivingEntity {
        &self.living_entity
    }

    pub fn get_mut_living_entity(&mut self) -> &mut LivingEntity {
        &mut self.living_entity
    }

    pub fn get_dialogs(&self) -> &Vec<String> {
        &self.dialogs
    }

    pub fn get_dialog_index(&self) -> usize {
        self.dialog_index
    }

    pub fn set_dialog_index(&mut self, dialog_index: usize) {
        if dialog_index < self.dialogs.len() {
            self.dialog_index = dialog_index;
        }
    }

    pub fn get_droppable_items_percentage(&self) -> &HashMap<Item, f32> {
        &self.droppable_items_percentage
    }
}

impl LivingEntityCombat for NPC {
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