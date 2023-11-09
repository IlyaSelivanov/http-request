use anyhow::Error;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{prelude::Rect, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, tui::Event};

pub mod url;

pub trait Component {
    #[allow(unused_variables)]
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<(), Error> {
        Ok(())
    }

    fn init(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> Result<Option<Action>, Error> {
        let r = match event {
            Some(Event::Key(key_event)) => self.handle_key_events(key_event)?,
            Some(Event::Mouse(mouse_event)) => self.handle_mouse_events(mouse_event)?,
            _ => None,
        };
        Ok(r)
    }

    #[allow(unused_variables)]
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, Error> {
        Ok(None)
    }

    #[allow(unused_variables)]
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Result<Option<Action>, Error> {
        Ok(None)
    }

    #[allow(unused_variables)]
    fn update(&mut self, action: Action) -> Result<Option<Action>, Error> {
        Ok(None)
    }

    fn render(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<(), Error>;
}
