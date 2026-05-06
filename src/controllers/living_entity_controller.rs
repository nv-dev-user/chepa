use crate::models::zone::Zone;
use crate::models::living_entity::LivingEntity;

pub fn change_zone(living_entity: &mut LivingEntity, new_zone: Zone) {
    if let Some(current_zone) = living_entity.get_zone().cloned() {
        living_entity.set_come_from(current_zone);
    }
    living_entity.set_zone(new_zone);
}