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
            let mut actions = services::action::get_actions(services::npc::get_npc_by_zone(self.player.as_ref().unwrap().get_living_entity().get_zone(), &mut self.npcs));
            Self::update(&mut self.player, &self.zones, &mut actions);
            Self::render(&self.player, &self.zones, &actions);
        }
    }

    fn update(player: &mut Option<Player>, zones: &Vec<Zone>, actions: &mut [Box<dyn Action + '_>]) {
        match player {
            Some(player) => services::input::handle_input(&mut player.get_mut_living_entity(), actions, zones),
            None => eprintln!("Error: Player not found"),
        }
        // services::input::handle_input(&self.player.as_ref().unwrap().get_mut_living_entity(), &actions);
    }

    fn render(player: &Option<Player>, zones: &Vec<Zone>, actions: &[Box<dyn Action + '_>]) {
        match player {
            Some(player) => render(&player.get_living_entity(), zones, actions),
            None => eprintln!("Error: Player not found"),
        }
        // render(&self.player.as_ref().unwrap().get_living_entity(), &self.zones, &actions);
    }
}

fn main() {
    Game::new().setup().run();
}
