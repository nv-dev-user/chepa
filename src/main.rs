use chepa::services::renderer::render_player_position;

use chepa::models::armor::Armor;
use chepa::models::weapon::Weapon;
use chepa::models::zone::Zone;
use chepa::models::npc::NPC;
use chepa::models::player::Player;

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
        let zones = match services::jsonparser::receive_data_from_file("./src/data/zones.json") {
            Ok(contenu) => load_zones(&contenu),
            Err(e) => Err(e),
        };
        println!("Zones chargées : {:?}", zones);

        let weapons = match services::jsonparser::receive_data_from_file("./src/data/weapons.json") {
            Ok(contenu) => load_zones(&contenu),
            Err(e) => Err(e),
        };
        println!("Armes chargées : {:?}", weapons);

        Game {
            weapons: Vec::new(),
            zones: Vec::new(),
            armors: Vec::new(),
            npcs: Vec::new(),
            player: Player::new(),
        }
    }

    pub fn update(&mut self) {

    }

    pub fn render(&self) {
        render_player_position(&self.player.get_living_entity(), &self.zones);
    }
}

fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.render();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
