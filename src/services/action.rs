use crate::game::Game;
use crate::models::player::Player;
use crate::models::npc::NPC;
use crate::models::zone::Zone;

pub trait Action {
    fn get_description(&self) -> String;
    fn execute(&self, game: &mut Game);
}

pub struct SpeakToNPCAction; //<'a> {game: &'a Game}
pub struct AttackAction {id_enemy: u32,}
pub struct UseItemAction; //<'a> {game: &'a Game}

impl Action for SpeakToNPCAction {
    fn get_description(&self) -> String {
        String::from("Speaking to the NPC")
    }
    fn execute(&self, game: &mut Game) {
        println!("Speaking to the NPC");
    }
}

impl AttackAction {
    pub fn new(id_enemy: u32) -> Self {
        Self { id_enemy }
    }
}

impl Action for AttackAction {
    fn get_description(&self) -> String {
        String::from("Attacking the target")
    }
    fn execute(&self, game: &mut Game) {
        game.attack_npc(1);
    }
}

impl Action for UseItemAction {
    fn get_description(&self) -> String {
        String::from("Using the item")
    }
    fn execute(&self, game: &mut Game) {
        println!("Using the item");
    }
}

pub fn define_actions (game: &mut Game) -> Vec<Box<dyn Action>> {
    vec![
        Box::new(SpeakToNPCAction),
        Box::new(AttackAction::new(1)), //::new(enemy)
        Box::new(UseItemAction),
    ]
}