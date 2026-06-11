use crate::models::living_entity::LivingEntity;
use crate::models::zone::Zone;
use crate::services::zone::{
    change_living_entity_zone,
    Direction,
};
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use crate::services::action::Action;
use std::time::Duration;

fn clear_pending_input_events() {
    while poll(Duration::from_millis(0)).unwrap_or(false) {
        let _ = read();
    }
}

pub fn handle_input(lve: &mut LivingEntity, actions: &mut [Box<dyn Action + '_>], zones: &Vec<Zone>) {
    if let Ok(Event::Key(key_event)) = read() {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('z') | KeyCode::Char('Z') => change_living_entity_zone(lve, Direction::Nord, zones),
                KeyCode::Char('q') | KeyCode::Char('Q') => change_living_entity_zone(lve, Direction::Ouest, zones),
                KeyCode::Char('s') | KeyCode::Char('S') => change_living_entity_zone(lve, Direction::Sud, zones),
                KeyCode::Char('d') | KeyCode::Char('D') => change_living_entity_zone(lve, Direction::Est, zones),
                KeyCode::Char(c) if c.is_ascii_digit() => {
                      let index = c.to_digit(10).unwrap_or(10) as usize;
                      if let Some(action) = actions.get_mut(index) {
                          action.execute();
                      } else {return;}
                 }
                _ => println!("Touche invalide")
            }
            clear_pending_input_events();
        }
    }
}
