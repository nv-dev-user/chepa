use crate::game::Game;

use crate::models::living_entity::LivingEntity;
use crate::models::zone::Zone;
use crate::models::spawn_group::SpawnGroup;
use crate::models::npc::NPC;

use crate::services::spawn::try_spawn_on_zone_entry;

pub enum Direction {
    Nord,
    Sud,
    Ouest,
    Est,
}

pub fn change_living_entity_zone(game: &mut Game, direction: Direction) { //lve: &mut LivingEntity, zones: Vec<Zone>
    let zones = game.get_zones().clone();
    let lve = game.get_player_mut().unwrap().get_mut_living_entity();
    let current_zone = lve.get_zone();
    match direction {
        Direction::Nord => {
            if let Some(zone_id) = match current_zone {
                Some(zone) => zone.get_north_zone_id(),
                None => None
            } {
                let new_zone = search_zone_by_id(zone_id, &zones).expect("La zone n'a pas été trouvée");
                let come_from = current_zone.expect("La zone de provenance n'a pas été trouvée").clone();
                lve.set_zone(new_zone);
                lve.set_come_from(&come_from);
                // println!("{}", zone_id);
                try_spawn(game, new_zone);
            } else {
                println!("Il n'y a pas de zone au nord");
            }
        },
        Direction::Sud => {
            if let Some(zone_id) = match current_zone {
                Some(zone) => zone.get_south_zone_id(),
                None => None
            } {
                let new_zone = search_zone_by_id(zone_id, &zones).expect("La zone n'a pas été trouvée");
                let come_from = current_zone.expect("La zone de provenance n'a pas été trouvée").clone();
                lve.set_zone(new_zone);
                lve.set_come_from(&come_from);
                // println!("{}", zone_id);
                try_spawn(game, new_zone);
            } else {
                println!("Il n'y a pas de zone au sud");
            }
        },
        Direction::Ouest => {
            if let Some(zone_id) = match current_zone {
                Some(zone) => zone.get_west_zone_id(),
                None => None
            } {
                let new_zone = search_zone_by_id(zone_id, &zones).expect("La zone n'a pas été trouvée");
                let come_from = current_zone.expect("La zone de provenance n'a pas été trouvée").clone();
                lve.set_zone(new_zone);
                lve.set_come_from(&come_from);
                // println!("{}", zone_id);
                try_spawn(game, new_zone);
            } else {
                println!("Il n'y a pas de zone à l'ouest");
            }
        },
        Direction::Est => {
            if let Some(zone_id) = match current_zone {
                Some(zone) => zone.get_east_zone_id(),
                None => None
            } {
                let new_zone = search_zone_by_id(zone_id, &zones).expect("La zone n'a pas été trouvée");
                let come_from = current_zone.expect("La zone de provenance n'a pas été trouvée").clone();
                lve.set_zone(new_zone);
                lve.set_come_from(&come_from);
                // println!("{}", zone_id);
                try_spawn(game, new_zone);
            } else {
                println!("Il n'y a pas de zone à l'est");
            }
        },
    }
}

pub fn search_zone_by_id(id: u32, zones: &Vec<Zone>) -> Option<&Zone> {
    for zone in zones {
        if zone.get_entity().get_id() == id {
            return Some(zone);
        }
    }
    None
}

fn try_spawn(game: &mut Game, zone: &Zone) {
    let result = try_spawn_on_zone_entry(
        zone,
        game.get_spawn_groups(),
        game.get_npcs()
    );

    if result.spawned {
        game.attack_npc(result.mob.unwrap().get_living_entity().get_entity().get_id());
    }
}
