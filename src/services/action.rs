pub trait Action {
    fn get_description(&self) -> String;
    fn execute(&self);
}

pub struct SpeakToNPCAction;
pub struct AttackAction;
pub struct UseItemAction;

impl Action for SpeakToNPCAction {
    fn get_description(&self) -> String {
        String::from("Speaking to the NPC")
    }
    fn execute(&self) {
        println!("Speaking to the NPC");
    }
}

impl Action for AttackAction {
    fn get_description(&self) -> String {
        String::from("Attacking the target")
    }
    fn execute(&self) {
        println!("Attacking the target");
    }
}

impl Action for UseItemAction {
    fn get_description(&self) -> String {
        String::from("Using the item")
    }
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