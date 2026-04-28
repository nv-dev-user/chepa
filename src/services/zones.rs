use crate::models::{living_entity::LivingEntity, zone::Zone};

pub fn searchZoneById(id: u32, zones: &Vec<Zone>) -> Option<&Zone> {
    for zone in zones {
        if zone.get_entity().get_id() == id {
            return Some(zone);
        }
    }
    None
}