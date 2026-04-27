use crate::models::zone::Zone;
use crate::models::living_entity::LivingEntity;

pub fn change_zone(living_entity: &mut LivingEntity, new_zone: Zone) {
    living_entity.set_come_from(living_entity.get_zone().clone());
    living_entity.set_zone(new_zone);
}