use anyhow::Error;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{prelude::Rect, Frame};

use crate::{action::Action, ui::Event};

pub trait Component {
    fn init(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn handle_event(&mut self, event: Option<Event>) -> Action {
        match event {
            Some(Event::Quit) => Action::Quit,
            Some(Event::Tick) => Action::Tick,
            Some(Event::Render) => Action::Render,
            Some(Event::Key(key)) => self.handle_key(key),
            Some(Event::Mouse(mouse)) => self.handle_mouse(mouse),
            _ => Action::None,
        }
    }

    #[allow(unused_variables)]
    fn handle_key(&mut self, key: KeyEvent) -> Action {
        Action::None
    }

    #[allow(unused_variables)]
    fn handle_mouse(&mut self, mouse: MouseEvent) -> Action {
        Action::None
    }

    #[allow(unused_variables)]
    fn update(&mut self, action: Action) -> Action {
        Action::None
    }

    fn render(&mut self, f: &mut Frame<'_>, rect: Rect);
}
