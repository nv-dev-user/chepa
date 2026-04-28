use crate::models::living_entity::LivingEntity;
use crate::models::zone::Zone;

pub enum Direction {
    Nord,
    Sud,
    Ouest,
    Est,
}

pub fn changeLivingEntityZone(lve: &mut LivingEntity, direction: Direction) {
    let current_zone = lve.get_zone();
    match direction {
        Direction::Nord => {
            if let Some(zone_id) = current_zone.get_north_zone_id() {
                // let new_zone = Zone::get_zone(zone_id).expect("La zone n'a pas été trouvée");
                // lve.set_current_zone(new_zone);
                println!("{}", zone_id);
            } else {
                println!("Il n'y a pas de zone au nord");
            }
        },
        Direction::Sud => {
            if let Some(zone_id) = current_zone.get_south_zone_id() {
                // let new_zone = Zone::get_zone(zone_id).expect("La zone n'a pas été trouvée");
                // lve.set_current_zone(new_zone);
                println!("{}", zone_id);
            } else {
                println!("Il n'y a pas de zone au sud");
            }
        },
        Direction::Ouest => {
            if let Some(zone_id) = current_zone.get_west_zone_id() {
                // let new_zone = Zone::get_zone(zone_id).expect("La zone n'a pas été trouvée");
                // lve.set_current_zone(new_zone);
                println!("{}", zone_id);
            } else {
                println!("Il n'y a pas de zone à l'ouest");
            }
        },
        Direction::Est => {
            if let Some(zone_id) = current_zone.get_east_zone_id() {
                // let new_zone = Zone::get_zone(zone_id).expect("La zone n'a pas été trouvée");
                // lve.set_current_zone(new_zone);
                println!("{}", zone_id);
            } else {
                println!("Il n'y a pas de zone à l'est");
            }
        },
    }
}