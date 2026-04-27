#[derive(Debug, serde::Deserialize)]
pub struct Entity {
    id: u32,
    name: String,
}

impl Entity {
    pub fn new(id: u32, name: String) -> Self {
        Entity { id, name }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}