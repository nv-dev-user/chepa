use crate::services::renderer::render;

use crate::models::armor::Armor;
use crate::models::entity::Entity;
use crate::models::living_entity::LivingEntity;
use crate::models::weapon::Weapon;
use crate::models::zone::Zone;
use crate::models::npc::NPC;
use crate::models::player::Player;

mod services;
mod models;

use services::jsonparser::load_zones;

struct Game {
    weapons: Vec<Weapon>,
    zones: Vec<Zone>,
    armors: Vec<Armor>,
    npcs: Vec<NPC>,
    player: Player,
}

impl Game {
    pub fn new() -> Self {
        // Get metadata from JSON file
        // Get entities form JSON files
        let zones = match services::jsonparser::receive_data_from_file("./data/zones.json") {
            Ok(contenu) => load_zones(&contenu),
            Err(e) => Err(e),
        };
        println!("Zones chargées : {:?}", zones);

        let weapons = match services::jsonparser::receive_data_from_file("./data/weapons.json") {
            Ok(contenu) => load_zones(&contenu),
            Err(e) => Err(e),
        };
        println!("Armes chargées : {:?}", weapons);

        Game {
            weapons: Vec::new(),
            zones: Vec::new(),
            armors: Vec::new(),
            npcs: Vec::new(),
            player: Player::new(make_living_entity(10, "Player", 0), 0),
        }
    }

    pub fn update(&mut self) {
        let actions  = services::action::default_actions();
        services::input::handle_input(self.player.get_mut_living_entity(), &actions);
    }

    pub fn render(&self) {
        println!("ougagagaagagbouga");
        render(&self.player.get_living_entity(), &self.zones, services::action::default_actions());
    }
}

fn make_living_entity(id: u32, name: &str, base_xp: u32) -> LivingEntity {
    let zone = Zone::new(
        Entity::new(1, "Zone de depart".to_string()),
        1,
        10,
        None,
        None,
        None,
        None,
        None,
    );

    LivingEntity::new(
        Entity::new(id, name.to_string()),
        100,
        12,
        10,
        8,
        base_xp,
        None,
        None,
        zone.clone(),
        zone,
    )
}

fn main() {
    let mut game = Game::new();

    loop {
        game.render();
        game.update();
    }
}
