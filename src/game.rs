use crate::services::jsonparser;
use crate::services::action;
use crate::services::input;
use crate::services::renderer::render;

use crate::models::armor::Armor;
use crate::models::weapon::Weapon;
use crate::models::zone::Zone;
use crate::models::npc::NPC;
use crate::models::player::Player;
use crate::models::fight::Fight;


pub struct Game {
    weapons: Vec<Weapon>,
    zones: Vec<Zone>,
    armors: Vec<Armor>,
    npcs: Vec<NPC>,
    player: Option<Player>,
    current_fight: Option<Fight>
}

impl Game {
    pub fn new() -> Self {
        Game {
            weapons: Vec::new(),
            zones: Vec::new(),
            armors: Vec::new(),
            npcs: Vec::new(),
            player: None,
            current_fight: None
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

    pub fn get_player(&self) -> Option<&Player> {
        self.player.as_ref()
    }

    pub fn get_player_mut(&mut self) -> Option<&mut Player> {
        self.player.as_mut()
    }

    pub fn search_npc_by_id(&mut self, id: u32) -> Option<&mut NPC> {
        for npc in &mut self.npcs {
            if npc.get_living_entity().get_entity().get_id() == id {
                return Some(npc)
            }
        }
        None
    }

    pub fn attack_npc(&mut self, npc_id: u32){
        let player = self.player.as_mut().unwrap();
        let npc = self
            .npcs
            .iter_mut()
            .find(|npc|
                npc.get_living_entity()
                   .get_entity()
                   .get_id() == npc_id
            );

        if let Some(npc) = npc {
            if player.fight(npc){
                println!("Vous avez vaincu {} !", npc.get_living_entity().get_entity().get_name());
            }
            else {
                println!("Vous avez été vaincu par {} !", npc.get_living_entity().get_entity().get_name());
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            let actions = action::define_actions(self);
            self.update(&actions);
            self.render(&actions);
        }
    }

    fn update(&mut self, actions: &Vec<Box<dyn action::Action>>) {
        let has_player = self.player.is_some();

        match has_player {
            true => input::handle_input(self, &actions),
            false => eprintln!("Error: Player not found"),
        }
        // services::input::handle_input(&self.player.as_ref().unwrap().get_mut_living_entity(), &actions);
    }

    fn render(&self, actions: &Vec<Box<dyn action::Action>>) {
        match self.player {
            Some(ref player) => render(&player.get_living_entity(), &self.zones, &actions),
            None => eprintln!("Error: Player not found"),
        }
        // render(&self.player.as_ref().unwrap().get_living_entity(), &self.zones, &actions);
    }
}