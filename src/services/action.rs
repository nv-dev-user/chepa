use crate::game::Game;
use crate::models::npc::NPC;

pub trait Action {
    fn get_description(&self) -> String;
    fn execute(&self, game: &mut Game);
}

pub struct SpeakToNPCAction {
    id_npc: u32,
    name_npc: String,
}
pub struct AttackAction {
    id_npc: u32,
    name_npc: String,
}

impl Action for SpeakToNPCAction {
    fn get_description(&self) -> String {
        format!("Parler à {}", self.name_npc)
    }

    fn execute(&self, game: &mut Game) {
        game.speak_with_npc(self.id_npc);
    }
}

impl AttackAction {
    pub fn new(id: u32, name: String) -> Self {
        Self { id_npc: id, name_npc: name }
    }
}

impl AttackAction {
    pub fn new(npc: NPC) -> Self {
        AttackAction { npc }
    }
}

impl Action for AttackAction {
    fn get_description(&self) -> String {
        format!("Attaquer {}", self.name_npc)
    }
    fn execute(&self, game: &mut Game) {
        game.attack_npc(self.id_npc);
    }
}

pub fn get_actions<'a>(npcs: Vec<&'a mut NPC>) -> Vec<Box<dyn Action + 'a>> {
    let mut actions: Vec<Box<dyn Action + 'a>> = Vec::new();
    for npc in npcs.iter() {
        actions.push(Box::new(AttackAction { 
            id_npc: (*npc).get_living_entity().get_entity().get_id(), 
            name_npc: (*npc).get_living_entity().get_entity().get_name().to_string(),
        }));
    }

    for npc in npcs {
        actions.push(Box::new(SpeakToNPCAction { 
            id_npc: (*npc).get_living_entity().get_entity().get_id(), 
            name_npc: (*npc).get_living_entity().get_entity().get_name().to_string(),
         }));
    }
    actions    
}