use super::item::Item;
use std::collections::HashMap;
use super::living_entity::LivingEntity;

pub struct PNJ {
    living_entity: LivingEntity,
    dialogs: Vec<String>,
    droppable_items_percentage : HashMap<Item, f32>,
}

impl PNJ {
    pub fn new(
        living_entity: LivingEntity, 
        dialogs: Vec<String>, 
        droppable_items_percentage: HashMap<Item, f32>
    ) -> Self {
        PNJ { 
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