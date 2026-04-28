use crate::models::living_entity::LivingEntity;
use crate::services::zone::{
    changeLivingEntityZone,
    Direction,
};
use crossterm::event::{read, Event, KeyCode, KeyEventKind};

pub fn handle_input(lve: &mut LivingEntity) {
    if let Ok(Event::Key(key_event)) = read() {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('z') | KeyCode::Char('Z') => changeLivingEntityZone(lve, Direction::Nord),
                KeyCode::Char('q') | KeyCode::Char('Q') => changeLivingEntityZone(lve, Direction::Ouest),
                KeyCode::Char('s') | KeyCode::Char('S') => changeLivingEntityZone(lve, Direction::Sud),
                KeyCode::Char('d') | KeyCode::Char('D') => changeLivingEntityZone(lve, Direction::Est),
                _ => {}
            }
        }
    }
}
