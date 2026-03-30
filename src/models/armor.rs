use super::entity::Entity;

pub struct Armor {
    entity: Entity,
    coef_def: u32,
}

impl Armor {
    pub fn new(entity: Entity, coef_def: u32) -> Self {
        Armor { entity, coef_def }
    }

    pub fn get_id(&self) -> u32 {
        self.entity.get_id()
    }

    pub fn get_name(&self) -> &str {
        self.entity.get_name()
    }

    pub fn get_coef_def(&self) -> u32 {
        self.coef_def
    }
}