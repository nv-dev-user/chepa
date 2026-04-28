pub trait Action {
    fn execute(&self);
}

pub struct SpeakToNPCAction;
pub struct AttackAction;
pub struct UseItemAction;

impl Action for SpeakToNPCAction {
    fn execute(&self) {
        println!("Speaking to the NPC");
    }
}

impl Action for AttackAction {
    fn execute(&self) {
        println!("Attacking the target");
    }
}

impl Action for UseItemAction {
    fn execute(&self) {
        println!("Using the item");
    }
}

pub fn default_actions() -> Vec<Box<dyn Action>> {
    vec![
        Box::new(SpeakToNPCAction),
        Box::new(AttackAction),
        Box::new(UseItemAction),
    ]
}