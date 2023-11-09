use ratatui::widgets::ListState;

/// A generic struct representing a stateful list.
/// This struct is used to represent a list of items that can be scrolled through and selected.
/// It keeps track of the current selected index and provides methods for updating the list and
/// selecting items.
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    #[allow(dead_code)]
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn unselect(&mut self) {
        self.state.select(None);
    }
}

#[allow(dead_code)]
pub struct Method {
    pub methods: StatefulList<String>,
}

impl Method {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            methods: StatefulList::with_items(vec![
                String::from("GET"),
                String::from("POST"),
                String::from("PUT"),
                String::from("DELETE"),
            ]),
        }
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
    fn test_select_next_method() {
        let mut method = Method::new();
        assert_eq!(method.methods.state.selected(), None);
        method.select_next_method();
        assert_eq!(method.methods.state.selected(), Some(0));
        method.select_next_method();
        assert_eq!(method.methods.state.selected(), Some(1));
        method.select_next_method();
        assert_eq!(method.methods.state.selected(), Some(2));
        method.select_next_method();
        assert_eq!(method.methods.state.selected(), Some(3));
        method.select_next_method();
        assert_eq!(method.methods.state.selected(), Some(0));
    }

    #[test]
    fn test_select_previous_method() {
        let mut method = Method::new();
        assert_eq!(method.methods.state.selected(), None);
        method.select_previous_method();
        assert_eq!(method.methods.state.selected(), Some(0));
        method.select_previous_method();
        assert_eq!(method.methods.state.selected(), Some(3));
        method.select_previous_method();
        assert_eq!(method.methods.state.selected(), Some(2));
        method.select_previous_method();
        assert_eq!(method.methods.state.selected(), Some(1));
        method.select_previous_method();
        assert_eq!(method.methods.state.selected(), Some(0));
    }
}
