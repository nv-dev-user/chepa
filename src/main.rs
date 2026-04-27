use chepa::services::renderer::render_player_position;

use chepa::models::armor::Armor;
use chepa::models::entity::Entity;
use chepa::models::living_entity::LivingEntity;
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
            player: Player::new(make_living_entity(10, "Player", 0), 0),
        }
    }

    pub fn update(&mut self) {

    }

    pub fn render(&self) {
        render_player_position(&self.player.get_living_entity(), &self.zones);
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
        game.update();
        game.render();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
