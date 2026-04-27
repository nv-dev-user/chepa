use crate::models::{living_entity::LivingEntity, zone::Zone};

pub fn render_player_position(player: &LivingEntity, zones: &Vec<Zone>) {
    // Render around the zone

    let north = player.get_zone().get_north_zone_id();
    let east = player.get_zone().get_east_zone_id();
    let west = player.get_zone().get_west_zone_id();
    let south = player.get_zone().get_south_zone_id();

    println!("North: {:?}", north);
    print!("East: {:?} ", east);
    render_player(&player, [north, east, south, west]);
    println!(" West: {:?}", west);
    println!("South: {:?}", south);
}

fn render_player(player: &LivingEntity, zones: [Option<u32>; 4]) {
    let player_zone_id = player.get_come_from().get_entity().get_id();

    if zones[0].is_some() && player_zone_id == zones[0].unwrap() {
        print!("▼");
    }
    else if zones[1].is_some() && player_zone_id == zones[1].unwrap() {
        print!("◀");
    }
    else if zones[2].is_some() && player_zone_id == zones[2].unwrap() {
        print!("▲");
    }
    else if zones[3].is_some() && player_zone_id == zones[3].unwrap() {
        print!("▶");
    }
    else {
        print!("●");
    }
}

#[cfg(test)]
mod tests {
    use crate::models::entity::Entity;

    use super::*;

    #[test]
    fn test_render_player_position() {
        let zones = Vec::from([
            Zone::new(Entity::new(1, "Zone 1".to_string()), 1, 10, None, Some(2), None, None, None),
            Zone::new(Entity::new(2, "Zone 2".to_string()), 1, 10, None, None, Some(1), None, None),
            Zone::new(Entity::new(3, "Zone 3".to_string()), 1, 10, None, None, None, Some(4), None),
            Zone::new(Entity::new(4, "Zone 4".to_string()), 1, 10, None, None, None, None, Some(3))
        ]);
        let player = LivingEntity::new(
            Entity::new(1, "Player".to_string()),
            100, 10, 5, 5, 0, None, None,
            Zone::new(Entity::new(1, "Zone 1".to_string()), 1, 10, None, Some(2), None, None, None),
            Zone::new(Entity::new(2, "Zone 2".to_string()), 1, 10, None, None, Some(1), None, None)
        );

        render_player_position(&player, &zones);
        // Assertions ici
    }
}