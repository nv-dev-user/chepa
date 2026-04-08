use super::item::Item;
use std::collections::HashMap;
use super::living_entity::LivingEntity;

pub struct NPC {
    living_entity: LivingEntity,
    dialogs: Vec<String>,
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
            droppable_items_percentage }
    }

    pub fn get_living_entity(&self) -> &LivingEntity {
        &self.living_entity
    }

    pub fn get_dialogs(&self) -> &Vec<String> {
        &self.dialogs
    }

    pub fn get_droppable_items_percentage(&self) -> &HashMap<Item, f32> {
        &self.droppable_items_percentage
    }
}