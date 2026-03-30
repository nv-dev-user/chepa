use super::entity::Entity;

pub struct Weapon {
    entity: Entity,
    coef_damage: u32,
}

impl Weapon {
    pub fn new(id: u32, name: String, coef_damage: u32) -> Self {
        Weapon { entity: Entity::new(id, name), coef_damage }
    }

    pub fn get_id(&self) -> u32 {
        self.entity.get_id()
    }

    pub fn get_name(&self) -> &str {
        self.entity.get_name()
    }

    pub fn get_coef_damage(&self) -> u32 {
        self.coef_damage
    }
}