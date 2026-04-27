use super::equipment::Equipment;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Armor {
    equipment: Equipment,
}

impl Armor {
    pub fn new(id: u32, name: String, coef_modifier: u32) -> Self {
        Armor { equipment: Equipment::new(id, name, coef_modifier) }
    }

    pub fn new_from_equipment(equipment: Equipment) -> Self {
        Armor { equipment }
    }

    pub fn get_id(&self) -> u32 {
        self.equipment.get_id()
    }

    pub fn get_name(&self) -> &str {
        self.equipment.get_name()
    }
}