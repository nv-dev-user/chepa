use crate::services::jsonparser;

use crate::models::armor::Armor;
use crate::models::weapon::Weapon;
use crate::models::zone::Zone;
use crate::models::npc::NPC;
use crate::models::player::Player;
use crate::services::action::Action;
use crate::services::renderer::render;

mod services;
mod models;


struct Game {
    weapons: Vec<Weapon>,
    zones: Vec<Zone>,
    armors: Vec<Armor>,
    npcs: Vec<NPC>,
    player: Option<Player>
}

impl Game {
    pub fn new() -> Self {
        Game {
            weapons: Vec::new(),
            zones: Vec::new(),
            armors: Vec::new(),
            npcs: Vec::new(),
            player: None
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

        match jsonparser::receive_data_from_file("data/player.json") {
            Ok(player_data) => match jsonparser::load_player(&player_data, &self.zones) {
                Ok(player) => self.player = Some(player),
                Err(e) => eprintln!("Error parsing player: {}", e),
            },
            Err(e) => eprintln!("Error loading player: {}", e),
        }

        self
    }

    pub fn run(&mut self) {
        loop {
            let actions = services::action::default_actions();
            self.update(&actions);
            self.render(&actions);
        }
    }

    fn update(&mut self, actions: &Vec<Box<dyn Action>>) {
        match self.player {
            Some(ref mut player) => services::input::handle_input(&mut player.get_mut_living_entity(), &actions),
            None => eprintln!("Error: Player not found"),
        }
        // services::input::handle_input(&self.player.as_ref().unwrap().get_mut_living_entity(), &actions);
    }

    fn render(&self, actions: &Vec<Box<dyn Action>>) {
        match self.player {
            Some(ref player) => render(&player.get_living_entity(), &self.zones, &actions),
            None => eprintln!("Error: Player not found"),
        }
        // render(&self.player.as_ref().unwrap().get_living_entity(), &self.zones, &actions);
    }
}

fn main() {
    Game::new().setup().run();
}
