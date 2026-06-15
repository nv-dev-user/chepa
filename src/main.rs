mod game;
mod models;
mod services;

use game::Game;

fn main() {
    Game::new().setup().run();
}
