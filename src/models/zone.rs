use super::entity::Entity;

pub struct Zone {
    entity: Entity,
    base_level: u32,
    spawn_rate: u8,
    spawn_group_id: u32,
    north_zone_id: Option<u32>,
    south_zone_id: Option<u32>,
    east_zone_id: Option<u32>,
    west_zone_id: Option<u32>,
}

impl Zone {
    pub fn new(
        entity: Entity,
        base_level: u32,
        spawn_rate: u8,
        spawn_group_id: u32,
        north_zone_id: Option<u32>,
        south_zone_id: Option<u32>,
        east_zone_id: Option<u32>,
        west_zone_id: Option<u32>
    ) -> Self {
        Zone {
            entity,
            base_level,
            spawn_rate,
            spawn_group_id,
            north_zone_id,
            south_zone_id,
            east_zone_id,
            west_zone_id
        }
    }

    pub fn get_entity(&self) -> &Entity {
        &self.entity
    }

    pub fn get_base_level(&self) -> u32 {
        self.base_level
    }

    pub fn get_spawn_rate(&self) -> u8 {
        self.spawn_rate
    }

    pub fn get_spawn_group_id(&self) -> u32 {
        self.spawn_group_id
    }

    pub fn get_north_zone_id(&self) -> Option<u32> {
        self.north_zone_id
    }

    pub fn get_south_zone_id(&self) -> Option<u32> {
        self.south_zone_id
    }

    pub fn get_east_zone_id(&self) -> Option<u32> {
        self.east_zone_id
    }

    pub fn get_west_zone_id(&self) -> Option<u32> {
        self.west_zone_id
    }
}