pub struct Input {
    pub input: String,
    pub cursor_position: usize,
}

impl Input {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor_position: 0,
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

    pub fn value(&self) -> &str {
        self.input.as_str()
    }

    pub fn handle_event(&mut self, event: &crossterm::event::Event) {
        if let crossterm::event::Event::Key(key) = event {
            match key.code {
                crossterm::event::KeyCode::Left => self.move_cursor_left(),
                crossterm::event::KeyCode::Right => self.move_cursor_right(),
                crossterm::event::KeyCode::Backspace => self.delete_char(),
                crossterm::event::KeyCode::Char(c) => self.enter_char(c),
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_input() {
        let input = Input::new();
        assert_eq!(input.input, "");
        assert_eq!(input.cursor_position, 0);
    }

    #[test]
    fn test_move_cursor_left() {
        let mut input = Input::new();
        input.input = String::from("hello");
        input.cursor_position = 3;

        input.move_cursor_left();
        assert_eq!(input.cursor_position, 2);

        input.move_cursor_left();
        assert_eq!(input.cursor_position, 1);

        input.move_cursor_left();
        assert_eq!(input.cursor_position, 0);

        input.move_cursor_left();
        assert_eq!(input.cursor_position, 0);
    }

    #[test]
    fn test_move_cursor_right() {
        let mut input = Input::new();
        input.input = String::from("hello");
        input.cursor_position = 1;

        input.move_cursor_right();
        assert_eq!(input.cursor_position, 2);

        input.move_cursor_right();
        assert_eq!(input.cursor_position, 3);

        input.move_cursor_right();
        assert_eq!(input.cursor_position, 4);

        input.move_cursor_right();
        assert_eq!(input.cursor_position, 5);

        input.move_cursor_right();
        assert_eq!(input.cursor_position, 5);
    }

    #[test]
    fn test_enter_char() {
        let mut input = Input::new();
        input.enter_char('h');
        assert_eq!(input.input, "h");
        assert_eq!(input.cursor_position, 1);

        input.enter_char('e');
        assert_eq!(input.input, "he");
        assert_eq!(input.cursor_position, 2);

        input.enter_char('l');
        assert_eq!(input.input, "hel");
        assert_eq!(input.cursor_position, 3);

        input.enter_char('l');
        assert_eq!(input.input, "hell");
        assert_eq!(input.cursor_position, 4);

        input.enter_char('o');
        assert_eq!(input.input, "hello");
        assert_eq!(input.cursor_position, 5);
    }

    #[test]
    fn test_delete_char() {
        let mut input = Input::new();
        input.input = String::from("hello");
        input.cursor_position = 3;

        input.delete_char();
        assert_eq!(input.input, "helo");
        assert_eq!(input.cursor_position, 2);

        input.delete_char();
        assert_eq!(input.input, "hlo");
        assert_eq!(input.cursor_position, 1);

        input.delete_char();
        assert_eq!(input.input, "lo");
        assert_eq!(input.cursor_position, 0);

        input.delete_char();
        assert_eq!(input.input, "lo");
        assert_eq!(input.cursor_position, 0);
    }

    #[test]
    fn test_clamp_cursor() {
        let input = Input::new();
        assert_eq!(input.clamp_cursor(0), 0);
        assert_eq!(input.clamp_cursor(1), 0);

        let mut input = Input::new();
        input.input = String::from("hello");
        assert_eq!(input.clamp_cursor(0), 0);
        assert_eq!(input.clamp_cursor(1), 1);
        assert_eq!(input.clamp_cursor(2), 2);
        assert_eq!(input.clamp_cursor(3), 3);
        assert_eq!(input.clamp_cursor(4), 4);
        assert_eq!(input.clamp_cursor(5), 5);
        assert_eq!(input.clamp_cursor(6), 5);
    }

    #[test]
    fn test_reset_cursor() {
        let mut input = Input::new();
        input.cursor_position = 3;

        input.reset_cursor();
        assert_eq!(input.cursor_position, 0);
    }

    #[test]
    fn test_value() {
        let mut input = Input::new();
        input.input = String::from("hello");

        assert_eq!(input.value(), "hello");
    }
}
