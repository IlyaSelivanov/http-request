use crossterm::event::{KeyCode, KeyEvent};

use super::{tui::Event, App, InputMode};

pub async fn update(app: &mut App, event: Event) -> Result<(), Box<dyn std::error::Error>> {
    if let Event::Key(key) = event {
        update_key(app, key).await;
    }
    Ok(())
}

pub async fn update_key(app: &mut App, key_event: KeyEvent) {
    match app.input_mode {
        InputMode::Normal => match key_event.code {
            KeyCode::Char('e') => {
                app.input_mode = InputMode::Editing;
            }
            KeyCode::Char('q') => {
                app.quit();
            }
            _ => {}
        },
        InputMode::Editing => match key_event.code {
            KeyCode::Enter => app.submit_message().await,
            KeyCode::Char(to_insert) => {
                app.enter_char(to_insert);
            }
            KeyCode::Backspace => {
                app.delete_char();
            }
            KeyCode::Left => {
                app.move_cursor_left();
            }
            KeyCode::Right => {
                app.move_cursor_right();
            }
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
            }
            KeyCode::Up => {
                app.select_next_method();
            }
            KeyCode::Down => {
                app.select_previous_method();
            }
            _ => {}
        },
    }
}
