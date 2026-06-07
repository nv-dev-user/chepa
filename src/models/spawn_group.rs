use std::collections::HashMap;

#[derive(Debug)]
pub struct SpawnGroup {
    id: u32,
    npcs: HashMap<u32, f32>,
}

impl SpawnGroup {
    pub fn new(id: u32, npcs: HashMap<u32, f32>) -> Self {
        SpawnGroup { id, npcs }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_npcs(&self) -> &HashMap<u32, f32> {
        &self.npcs
    }
}
