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

pub async fn main_ui() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new();
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    loop {
        let event = tui.events.next().await?;

        update(&mut app, event).await?;

        tui.draw(&mut app)?;

        if app.should_quit {
            break;
        }
    }

    tui.exit()?;
    Ok(())
}
