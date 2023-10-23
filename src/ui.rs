use std::{error::Error, io};

use ratatui::prelude::*;

mod app;
pub use app::*;

mod renderer;
pub use renderer::*;

mod event_handler;
pub use event_handler::*;

mod tui;
pub use tui::*;

mod update;
pub use update::*;

use crate::request::Request;

pub async fn main_ui(request: Request) -> Result<(), Box<dyn Error>> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new();
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;
        match tui.events.next().await? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            // Event::Mouse(_) => {}
            // Event::Resize(_, _) => {}
            _ => {}
        };
    }

    tui.exit()?;
    Ok(())
}
