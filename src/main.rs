use chepa::services::renderer::render_player_position;

use chepa::models::armor::Armor;
use chepa::models::weapon::Weapon;
use chepa::models::zone::Zone;
use chepa::models::npc::NPC;
use chepa::models::player::Player;

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
