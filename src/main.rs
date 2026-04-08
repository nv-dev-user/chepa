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
    players: Vec<Player>,
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
            players: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        println!("Updating game state...");
        // Here you can add code to update the game logic, handle user input, etc.
    }

    pub fn render(&self) {
        println!("Rendering game...");
        // Here you can add code to draw the game world, UI, etc.
    }
}

fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.render();
    }
}
