pub struct Input {
    pub input: String,
    pub cursor_position: usize,
}

impl Input {
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
