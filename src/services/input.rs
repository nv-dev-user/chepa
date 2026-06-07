use crate::game::Game;
use crate::models::living_entity::LivingEntity;
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

pub fn handle_input(game: &mut Game, actions: &[Box<dyn Action>]) {
    let lve = game.get_player_mut().unwrap().get_mut_living_entity();
    if let Ok(Event::Key(key_event)) = read() {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('z') | KeyCode::Char('Z') => change_living_entity_zone(lve, Direction::Nord),
                KeyCode::Char('q') | KeyCode::Char('Q') => change_living_entity_zone(lve, Direction::Ouest),
                KeyCode::Char('s') | KeyCode::Char('S') => change_living_entity_zone(lve, Direction::Sud),
                KeyCode::Char('d') | KeyCode::Char('D') => change_living_entity_zone(lve, Direction::Est),
                KeyCode::Char(c) if c.is_ascii_digit() => {
                      let index = c.to_digit(10).unwrap_or(10) as usize;
                      if let Some(action) = actions.get(index) {
                          action.execute(game);
                      } else {return;}
                 }
                _ => println!("Touche invalide")
            }
            clear_pending_input_events();
        }
    }
}
