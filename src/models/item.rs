use super::entity::Entity;

enum ItemType {
    Weapon,
    Armor,
}

pub struct Item {
    entity: Entity,
    item_type: ItemType,
    item_id: u32,
}

impl Item {
    pub fn new(entity: Entity, item_type: ItemType, item_id: u32) -> Self {
        Item { entity, item_type, item_id }
    }

    pub fn get_entity(&self) -> &Entity {
        &self.entity
    }

    pub fn get_item_id(&self) -> &u32 {
        &self.item_id
    }

    pub fn get_item_type(&self) -> &ItemType {
        &self.item_type
    }
}