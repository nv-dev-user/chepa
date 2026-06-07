use crate::models::{living_entity::LivingEntity, zone::Zone};
use crate::services::{zone::search_zone_by_id, action};

use crossterm::terminal;

pub fn render(player: &LivingEntity, zones: &Vec<Zone>, actions: &Vec<Box<dyn action::Action>>){
    let (cols, rows) = terminal::size().unwrap();
    let separation = "=".repeat(cols as usize);
    println!("{}\n\n", separation);
    render_player_position(player, zones);
    println!("\n\n");
    render_possible_actions(actions);
    println!("\n{}", separation);
}

pub fn render_fight(){
    let (cols, rows) = terminal::size().unwrap();
    let separation = "=".repeat(cols as usize);
    println!("{}\n\n", separation);



    let top_action = "Attaquer (Z)";
    let left_action = "Defendre (Q)";
    let right_action = "Fuir (D)";

    let space_left = " ".repeat((cols as usize)/2 - (left_action.chars().count()+6));
    let space_top = " ".repeat((cols as usize)/2 - (top_action.chars().count())/2);

    println!("{}{}\n", space_top, top_action);
    print!("{}{}", space_left, left_action);
    print!("      ●      ");
    println!("{}\n", right_action);


    
    println!("\n{}", separation);
}

pub fn render_player_position(player: &LivingEntity, zones: &Vec<Zone>) {
    // Render around the zone

    // Find id and name of each neighbor zones
    let north = player.get_zone().and_then(|zone| zone.get_north_zone_id());
    let north_name = north
        .and_then(|i| search_zone_by_id(i, zones))
        .map(|zone| format!("North (Z): {}", zone.get_entity().get_name()))
        .unwrap_or_default();

    let east = player.get_zone().and_then(|zone| zone.get_east_zone_id());
    let east_name = east
        .and_then(|i| search_zone_by_id(i, zones))
        .map(|zone| format!("East (D): {}", zone.get_entity().get_name()))
        .unwrap_or_default();

    let west = player.get_zone().and_then(|zone| zone.get_west_zone_id());
    let west_name = west
        .and_then(|i| search_zone_by_id(i, zones))
        .map(|zone| format!("West (Q): {}", zone.get_entity().get_name()))
        .unwrap_or_default();

    let south = player.get_zone().and_then(|zone| zone.get_south_zone_id());
    let south_name = south
        .and_then(|i| search_zone_by_id(i, zones))
        .map(|zone| format!("South (S): {}", zone.get_entity().get_name()))
        .unwrap_or_default();

    // Calculate spaces number before neighbor zones' print
    let (cols, rows) = terminal::size().unwrap();
    let space_west_and_east = " ".repeat((cols as usize)/2 - (west_name.chars().count()+6));
    let space_north = " ".repeat((cols as usize)/2 - (north_name.chars().count())/2);
    let space_south = " ".repeat((cols as usize)/2 - (south_name.chars().count())/2);
    let center = render_arrow_for_position(&player, [north, east, south, west]);

    // Print
    println!("{}{}\n", space_north, north_name);
    print!("{}{}", space_west_and_east, west_name);
    print!("{}", center);
    println!("{}\n", east_name);
    println!("{}{}", space_south, south_name);
}

fn render_possible_actions(actions: &Vec<Box<dyn action::Action>>){
    for (i, value) in actions.iter().enumerate() {
        println!("        {} : {}", i, value.get_description());
    }
}

fn render_arrow_for_position(player: &LivingEntity, zones: [Option<u32>; 4]) -> String {
    let player_zone_id = player.get_come_from().get_entity().get_id();

    if zones[0].is_some() && player_zone_id == zones[0].unwrap() {
        String::from("      ▼      ")
    }
    else if zones[1].is_some() && player_zone_id == zones[1].unwrap() {
        String::from("      ◀      ")
    }
    else if zones[2].is_some() && player_zone_id == zones[2].unwrap() {
        String::from("      ▲      ")
    }
    else if zones[3].is_some() && player_zone_id == zones[3].unwrap() {
        String::from("      ▶      ")
    }
    else {
        String::from("      ●      ")
    }
}

#[cfg(test)]
mod tests {
    use crate::models::entity::Entity;

    use super::*;

    #[test]
    fn test_render_player_position() {
        let zones = Vec::from([
            Zone::new(Entity::new(1, "Zone 1".to_string()), 1, 10, None, Some(2), Some(1), Some(4), Some(3)),
            Zone::new(Entity::new(2, "Zone 2".to_string()), 1, 10, None, None, Some(1), None, None),
            Zone::new(Entity::new(3, "Zone 3".to_string()), 1, 10, None, None, None, Some(4), None),
            Zone::new(Entity::new(4, "Zone 4".to_string()), 1, 10, None, None, None, None, Some(3))

            // Zone::new(Entity::new(1, "Zone 1".to_string()), 1, 10, None, Some(2), Some(1), Some(4), Some(3)),
            // Zone::new(Entity::new(2, "Zone 2".to_string()), 1, 10, None, Some(2), Some(1), Some(4), Some(3)),
            // Zone::new(Entity::new(3, "Zone 3".to_string()), 1, 10, None, Some(2), Some(1), Some(4), Some(3)),
            // Zone::new(Entity::new(4, "Zone 4".to_string()), 1, 10, None, Some(2), Some(1), Some(4), Some(3))
        ]);
        let player = LivingEntity::new(
            Entity::new(1, "Player".to_string()),
            100, 10, 5, 5, 0, None, None,
            Some(Zone::new(Entity::new(1, "Zone 1".to_string()), 1, 10, None, Some(2), Some(1), Some(4), Some(3))),
            Zone::new(Entity::new(3, "Zone 2".to_string()), 1, 10, None, None, Some(1), None, None)
        );
        let actions: Vec<Box<dyn action::Action>> = vec![
            Box::new(action::SpeakToNPCAction),
            Box::new(action::UseItemAction),
            // Box::new(action::AttackAction)
        ];

        render(&player, &zones, &actions);
        // Assertions ici
    }

    #[test]
    fn test_render_fight() {
        render_fight();
        // Assertions ici
    }

    #[test]
    fn test_render_possible_actions() {
        let actions: Vec<Box<dyn action::Action>> = vec![
            Box::new(action::SpeakToNPCAction),
            Box::new(action::UseItemAction),
            // Box::new(action::AttackAction)
        ];

        render_possible_actions(&actions);
        // Assertions ici
    }

    #[test]
    fn test_render() {
        let actions: Vec<Box<dyn action::Action>> = vec![
            Box::new(action::SpeakToNPCAction),
            Box::new(action::UseItemAction),
            // Box::new(action::AttackAction)
        ];

        let zones = Vec::from([
            Zone::new(Entity::new(1, "Zone 1".to_string()), 1, 10, None, Some(2), Some(1), Some(4), Some(3)),
            Zone::new(Entity::new(2, "Zone 2".to_string()), 1, 10, None, None, Some(1), None, None),
            Zone::new(Entity::new(3, "Zone 3".to_string()), 1, 10, None, None, None, Some(4), None),
            Zone::new(Entity::new(4, "Zone 4".to_string()), 1, 10, None, None, None, None, Some(3))

            // Zone::new(Entity::new(1, "Zone 1".to_string()), 1, 10, None, Some(2), Some(1), Some(4), Some(3)),
            // Zone::new(Entity::new(2, "Zone 2".to_string()), 1, 10, None, Some(2), Some(1), Some(4), Some(3)),
            // Zone::new(Entity::new(3, "Zone 3".to_string()), 1, 10, None, Some(2), Some(1), Some(4), Some(3)),
            // Zone::new(Entity::new(4, "Zone 4".to_string()), 1, 10, None, Some(2), Some(1), Some(4), Some(3))
        ]);
        let player = LivingEntity::new(
            Entity::new(1, "Player".to_string()),
            100, 10, 5, 5, 0, None, None,
            Some(Zone::new(Entity::new(1, "Zone 1".to_string()), 1, 10, None, Some(2), Some(1), Some(4), Some(3))),
            Zone::new(Entity::new(3, "Zone 2".to_string()), 1, 10, None, None, Some(1), None, None)
        );

        render_possible_actions(&actions);
        // Assertions ici
    }
}