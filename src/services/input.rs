use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use crate::services::action::Action;

pub fn handle_input(actions: &[Box<dyn Action>]) {
    if let Ok(Event::Key(key_event)) = read() {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('z') | KeyCode::Char('Z') => println!("Action : haut"),
                KeyCode::Char('q') | KeyCode::Char('Q') => println!("Action : gauche"),
                KeyCode::Char('s') | KeyCode::Char('S') => println!("Action : bas"),
                KeyCode::Char('d') | KeyCode::Char('D') => println!("Action : droite"),
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
