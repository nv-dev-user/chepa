use crossterm::event::{read, Event, KeyCode, KeyEventKind};

pub fn handle_input() {
    if let Ok(Event::Key(key_event)) = read() {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Char('z') | KeyCode::Char('Z') => println!("Action : haut"),
                KeyCode::Char('q') | KeyCode::Char('Q') => println!("Action : gauche"),
                KeyCode::Char('s') | KeyCode::Char('S') => println!("Action : bas"),
                KeyCode::Char('d') | KeyCode::Char('D') => println!("Action : droite"),
                _ => {}
            }
        }
    }
}
