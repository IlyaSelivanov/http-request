use ratatui::widgets::ListState;

use crate::http_client::HttpRequest;

/// A generic struct representing a stateful list.
/// This struct is used to represent a list of items that can be scrolled through and selected.
/// It keeps track of the current selected index and provides methods for updating the list and
/// selecting items.
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

/// A struct representing the application.
/// This struct contains the main logic for the application, including handling user input and
/// displaying output to the user.
pub struct App {
    pub input: String,
    pub cursor_position: usize,
    pub input_mode: InputMode,
    pub messages: Vec<String>,
    pub should_quit: bool,
    pub methods: StatefulList<String>,
}

#[derive(PartialEq, Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

impl App {
    pub fn new() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            cursor_position: 0,
            should_quit: false,
            methods: StatefulList::with_items(vec![
                String::from("GET"),
                String::from("POST"),
                String::from("PUT"),
                String::from("DELETE"),
            ]),
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);

        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    pub async fn submit_message(&mut self) {
        self.messages.push(self.input.clone());

        let request = HttpRequest::new(crate::http_client::HttpMethod::Get, self.input.as_str());
        let response = request.send().await;

        self.messages.push(response.status_code.to_string());

        self.input.clear();
        self.reset_cursor();
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn select_next_method(&mut self) {
        self.methods.next();
    }

    pub fn select_previous_method(&mut self) {
        self.methods.previous();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_app() {
        let app = App::new();
        assert_eq!(app.input, String::new());
        assert_eq!(app.input_mode, InputMode::Normal);
        assert_eq!(app.messages, Vec::<String>::new());
        assert_eq!(app.cursor_position, 0);
        assert!(!app.should_quit);
    }

    #[test]
    fn test_move_cursor_left() {
        let mut app = App::new();
        app.input = String::from("hello");
        app.cursor_position = 5;

        app.move_cursor_left();
        assert_eq!(app.cursor_position, 4);
        app.move_cursor_left();
        assert_eq!(app.cursor_position, 3);
        app.move_cursor_left();
        assert_eq!(app.cursor_position, 2);
        app.move_cursor_left();
        assert_eq!(app.cursor_position, 1);
        app.move_cursor_left();
        assert_eq!(app.cursor_position, 0);
        app.move_cursor_left();
        assert_eq!(app.cursor_position, 0);
    }

    #[test]
    fn test_move_cursor_right() {
        let mut app = App::new();
        app.input = String::from("hello");
        app.move_cursor_right();
        assert_eq!(app.cursor_position, 1);
        app.move_cursor_right();
        assert_eq!(app.cursor_position, 2);
        app.move_cursor_right();
        assert_eq!(app.cursor_position, 3);
        app.move_cursor_right();
        assert_eq!(app.cursor_position, 4);
        app.move_cursor_right();
        assert_eq!(app.cursor_position, 5);
        app.move_cursor_right();
        assert_eq!(app.cursor_position, 5);
    }

    #[test]
    fn test_enter_char() {
        let mut app = App::new();
        app.enter_char('h');
        assert_eq!(app.input, String::from("h"));
        assert_eq!(app.cursor_position, 1);
        app.enter_char('e');
        assert_eq!(app.input, String::from("he"));
        assert_eq!(app.cursor_position, 2);
        app.enter_char('l');
        assert_eq!(app.input, String::from("hel"));
        assert_eq!(app.cursor_position, 3);
        app.enter_char('l');
        assert_eq!(app.input, String::from("hell"));
        assert_eq!(app.cursor_position, 4);
        app.enter_char('o');
        assert_eq!(app.input, String::from("hello"));
        assert_eq!(app.cursor_position, 5);
    }

    #[test]
    fn test_delete_char() {
        let mut app = App::new();
        app.input = String::from("hello");
        app.cursor_position = 5;
        app.delete_char();
        assert_eq!(app.input, String::from("hell"));
        assert_eq!(app.cursor_position, 4);
        app.delete_char();
        assert_eq!(app.input, String::from("hel"));
        assert_eq!(app.cursor_position, 3);
        app.delete_char();
        assert_eq!(app.input, String::from("he"));
        assert_eq!(app.cursor_position, 2);
        app.delete_char();
        assert_eq!(app.input, String::from("h"));
        assert_eq!(app.cursor_position, 1);
        app.delete_char();
        assert_eq!(app.input, String::new());
        assert_eq!(app.cursor_position, 0);
        app.delete_char();
        assert_eq!(app.input, String::new());
        assert_eq!(app.cursor_position, 0);
    }

    #[test]
    fn test_clamp_cursor() {
        let mut app = App::new();
        assert_eq!(app.clamp_cursor(0), 0);
        assert_eq!(app.clamp_cursor(1), 0);

        app.input = String::from("hello");
        assert_eq!(app.clamp_cursor(1), 1);
        assert_eq!(app.clamp_cursor(2), 2);
        assert_eq!(app.clamp_cursor(3), 3);
        assert_eq!(app.clamp_cursor(4), 4);
        assert_eq!(app.clamp_cursor(5), 5);
        assert_eq!(app.clamp_cursor(6), 5);
        assert_eq!(app.clamp_cursor(7), 5);
    }

    #[test]
    fn test_reset_cursor() {
        let mut app = App::new();
        app.cursor_position = 5;
        app.reset_cursor();
        assert_eq!(app.cursor_position, 0);
    }

    #[test]
    fn test_submit_message() {
        let mut app = App::new();
        app.input = String::from("hello");
        app.submit_message();
        assert_eq!(app.input, String::new());
        assert_eq!(app.messages, vec![String::from("hello")]);
        assert_eq!(app.cursor_position, 0);
    }

    #[test]
    fn test_quit() {
        let mut app = App::new();
        app.quit();
        assert!(app.should_quit);
    }
}
