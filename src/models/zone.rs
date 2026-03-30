use super::entity::Entity;
use super::directions::Directions;

pub struct Zone {
    entity: Entity,
    baseLevel: i32,
    spawnRate: u8,
    spawnGroup: i32,
    directions: Directions,
}

impl Zone {
    pub fn new(entity: Entity, baseLevel: i32, spawnRate: u8, spawnGroup: i32, directions: Directions) -> Self {
        Zone { entity, baseLevel, spawnRate, spawnGroup, directions }
    }

    pub fn get_entity(&self) -> &Entity {
        &self.entity
    }

    pub fn get_base_level(&self) -> i32 {
        self.baseLevel
    }

    pub fn get_spawn_rate(&self) -> u8 {
        self.spawnRate
    }

    pub fn get_spawn_group(&self) -> i32 {
        self.spawnGroup
    }

    pub fn get_z(&self) -> i32 {
        self.directions.get_z()
    }

    pub fn get_q(&self) -> i32 {
        self.directions.get_q()
    }

    pub fn get_s(&self) -> i32 {
        self.directions.get_s()
    }

    pub fn get_d(&self) -> i32 {
        self.directions.get_d()
    }
}