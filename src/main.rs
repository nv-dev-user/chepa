use crate::services::jsonparser;

use crate::models::armor::Armor;
use crate::models::weapon::Weapon;
use crate::models::zone::Zone;
use crate::models::npc::NPC;

mod services;
mod models;


struct Game {
    weapons: Vec<Weapon>,
    zones: Vec<Zone>,
    armors: Vec<Armor>,
    npcs: Vec<NPC>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            weapons: Vec::new(),
            zones: Vec::new(),
            armors: Vec::new(),
            npcs: Vec::new()
        }
    }

    pub fn setup(&mut self) -> &mut Self {
        // Load zones
        match jsonparser::receive_data_from_file("data/zones.json") {
            Ok(zones_data) => match jsonparser::load_zones(&zones_data) {
                Ok(zones) => self.zones = zones,
                Err(e) => eprintln!("Error parsing zones: {}", e),
            },
            Err(e) => eprintln!("Error loading zones: {}", e),
        }

        match jsonparser::receive_data_from_file("data/armors.json") {
            Ok(armors_data) => match jsonparser::load_armors(&armors_data) {
                Ok(armors) => self.armors = armors,
                Err(e) => eprintln!("Error parsing armors: {}", e),
            },
            Err(e) => eprintln!("Error loading armors: {}", e),
        }

        match jsonparser::receive_data_from_file("data/weapons.json") {
            Ok(weapons_data) => match jsonparser::load_weapons(&weapons_data) {
                Ok(weapons) => self.weapons = weapons,
                Err(e) => eprintln!("Error parsing weapons: {}", e),
            },
            Err(e) => eprintln!("Error loading weapons: {}", e),
        }

        match jsonparser::receive_data_from_file("data/npcs.json") {
            Ok(npcs_data) => match jsonparser::load_npc(&npcs_data, &self.zones) {
                Ok(npcs) => self.npcs = npcs,
                Err(e) => eprintln!("Error parsing npcs: {}", e),
            },
            Err(e) => eprintln!("Error loading npcs: {}", e),
        }

        self
    }

    pub fn run(&mut self) {
        loop {
            self.update();
            self.render();
        }
    }

    fn update(&mut self) {

    }

    fn render(&self) {

    }
}

fn main() {
    Game::new().setup().run();
}
