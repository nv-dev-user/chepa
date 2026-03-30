use super::entity::Entity;
use super::directions::Directions;

pub struct Zone {
    entity: Entity,
    base_level: i32,
    spawn_rate: u8,
    spawn_group: i32,
    directions: Directions,
}

impl Zone {
    pub fn new(entity: Entity, base_level: i32, spawn_rate: u8, spawn_group: i32, directions: Directions) -> Self {
        Zone { entity, base_level, spawn_rate, spawn_group, directions }
    }

    pub fn get_entity(&self) -> &Entity {
        &self.entity
    }

    pub fn get_base_level(&self) -> i32 {
        self.base_level
    }

    pub fn get_spawn_rate(&self) -> u8 {
        self.spawn_rate
    }

    pub fn get_spawn_group(&self) -> i32 {
        self.spawn_group
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