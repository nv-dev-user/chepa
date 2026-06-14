use crate::models::npc::NPC;

pub trait Action {
    fn get_description(&self) -> String;
    fn execute(&mut self);
}

pub struct SpeakToNPCAction<'a> {
    npc: &'a mut NPC,
}
pub struct AttackAction {
    npc: NPC,
}

impl<'a> Action for SpeakToNPCAction<'a> {
    fn get_description(&self) -> String {
        format!("Parler à {}", self.npc.get_living_entity().get_entity().get_name())
    }

    fn execute(&mut self) {
        let name = self.npc.get_living_entity().get_entity().get_name();
        let dialogs = self.npc.get_dialogs();
        if !dialogs.is_empty() {
            let idx = self.npc.get_dialog_index().min(dialogs.len() - 1);
            println!("{} dit: {}", name, dialogs[idx]);
            self.npc.set_dialog_index(idx + 1);
        } else {
            println!("{} n'a rien à dire.", name);
        }
    }
}

impl AttackAction {
    pub fn new(npc: NPC) -> Self {
        AttackAction { npc }
    }
}

impl Action for AttackAction {
    fn get_description(&self) -> String {
        format!("Attaquer {}", self.npc.get_living_entity().get_entity().get_name())
    }
    
    fn execute(&mut self) {
        println!("Attaquer {}", self.npc.get_living_entity().get_entity().get_name());
        // TODO - Utiliser le système de combat opérationnel dev par Martin
    }
}

pub fn get_actions<'a>(npcs: Vec<&'a mut NPC>) -> Vec<Box<dyn Action + 'a>> {
    let mut actions: Vec<Box<dyn Action + 'a>> = Vec::new();
    for npc in npcs.iter() {
        actions.push(Box::new(AttackAction { npc: (*npc).clone() }));
    }

    for npc in npcs {
        actions.push(Box::new(SpeakToNPCAction { npc }));
    }
    actions    
}