use crate::models::living_entity::LivingEntity;
use crate::services::zone::{
    changeLivingEntityZone,
    Direction,
};
use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use crate::services::action::Action;

pub fn handle_input(lve: &mut LivingEntity, actions: &[Box<dyn Action>]) {
    if let Ok(Event::Key(key_event)) = read() {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('z') | KeyCode::Char('Z') => changeLivingEntityZone(lve, Direction::Nord),
                KeyCode::Char('q') | KeyCode::Char('Q') => changeLivingEntityZone(lve, Direction::Ouest),
                KeyCode::Char('s') | KeyCode::Char('S') => changeLivingEntityZone(lve, Direction::Sud),
                KeyCode::Char('d') | KeyCode::Char('D') => changeLivingEntityZone(lve, Direction::Est),
                KeyCode::Char(c) if c.is_ascii_digit() => {
                      let index = c.to_digit(10).unwrap_or(10) as usize;
                      if let Some(action) = actions.get(index) {
                          action.execute();
                      } else {return;}
                 }
                _ => {}
            }
        }
    }
}
