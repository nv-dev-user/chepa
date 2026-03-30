use super::entity::Entity;
use super::zone::Zone;
use super::item::Item;

pub struct PNJ {
    entity: Entity,
    zone: Zone,
    drop: Vec<Item>,
    dialogs: Vec<String>,
    has_dropped: bool,
}

impl PNJ {
    pub fn new(entity: Entity, zone: Zone, drop: Vec<Item>, dialogs: Vec<String>) -> Self {
        PNJ { entity, zone, drop, dialogs, has_dropped: false }
    }

    pub fn get_entity(&self) -> &Entity {
        &self.entity
    }

    pub fn get_zone(&self) -> &Zone {
        &self.zone
    }

    pub fn get_drop(&self) -> &Vec<Item> {
        &self.drop
    }

    pub fn get_dialogs(&self) -> &Vec<String> {
        &self.dialogs
    }

    pub fn has_dropped(&self) -> bool {
        self.has_dropped
    }

    pub fn set_has_dropped(&mut self, value: bool) {
        self.has_dropped = value;
    }
}