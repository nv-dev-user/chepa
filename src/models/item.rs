use super::entity::Entity;

#[derive(Debug, serde::Deserialize)]
pub struct Item {
    entity: Entity,
}

impl Item {
    pub fn new(id: u32, name: String) -> Self {
        Item { entity: Entity::new(id, name) }
    }

    pub fn new_from_entity(entity: Entity) -> Self {
        Item { entity }
    }

    pub fn get_id(&self) -> u32 {
        self.entity.get_id()
    }

    pub fn get_name(&self) -> &str {
        self.entity.get_name()
    }
}