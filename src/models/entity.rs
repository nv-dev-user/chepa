struct Entity {
    id: u32,
    name: String,
}

impl Entity {
    fn new(id: u32, name: String) -> Self {
        Entity { id, name }
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}