use super::equipment::Equipment;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct Weapon {
    equipment: Equipment,
}

impl Weapon {
    pub fn new(id: u32, name: String, coef_modifier: u32) -> Self {
        Weapon { equipment: Equipment::new(id, name, coef_modifier) }
    }

    pub fn new_from_equipment(equipment: Equipment) -> Self {
        Weapon { equipment }
    }

    pub fn get_id(&self) -> u32 {
        self.equipment.get_id()
    }

    pub fn get_name(&self) -> &str {
        self.equipment.get_name()
    }

    pub fn get_coef_modifier(&self) -> u32 {
        self.equipment.get_coef_modifier()
    }
}
