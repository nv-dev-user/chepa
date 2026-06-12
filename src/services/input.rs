use crate::game::Game;
use crate::models::living_entity::LivingEntity;
use crate::models::zone::Zone;
use crate::services::zone::{
    change_living_entity_zone,
    Direction,
};
use crossterm::event::{poll, read, Event, KeyEvent, KeyCode, KeyEventKind};
use crate::services::action::Action;
use std::time::Duration;

fn clear_pending_input_events() {
    while poll(Duration::from_millis(0)).unwrap_or(false) {
        let _ = read();
    }
}

fn process_input(key: KeyEvent, game: &mut Game, actions: &[Box<dyn Action>]){
    match key.code {
        KeyCode::Char('z') | KeyCode::Char('Z') => change_living_entity_zone(game, Direction::Nord),
        KeyCode::Char('q') | KeyCode::Char('Q') => change_living_entity_zone(game, Direction::Ouest),
        KeyCode::Char('s') | KeyCode::Char('S') => change_living_entity_zone(game, Direction::Sud),
        KeyCode::Char('d') | KeyCode::Char('D') => change_living_entity_zone(game, Direction::Est),
        KeyCode::Char(c) if c.is_ascii_digit() => {
            let index = c.to_digit(10).unwrap_or(10) as usize;
            if let Some(action) = actions.get(index) {
                action.execute(game);
            } else {return;}
        }
        _ => println!("Touche invalide")
    }
}

pub fn handle_input(game: &mut Game, actions: &[Box<dyn Action>]) {
    if let Ok(Event::Key(key_event)) = read() {
        if key_event.kind == KeyEventKind::Press {
            process_input(key_event, game, actions);
            clear_pending_input_events();
        }
    }
}
