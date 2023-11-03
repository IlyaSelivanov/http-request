use std::error::Error;

mod app;
pub use app::*;

mod renderer;
pub use renderer::*;

mod tui;
pub use tui::*;

mod update;
pub use update::*;

pub async fn main_ui() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();

    let mut tui = tui::Tui::new()?.tick_rate(1.0).frame_rate(30.0);
    tui.enter()?;

    loop {
        let event = tui.next().await?; // blocks until next event

        if let Event::Render = event.clone() {
            // application render
            tui.draw(|f| {
                render(f, &mut app);
            })?;
        }

        // application update
        update(&mut app, event).await?;

        // application exit
        if app.should_quit {
            break;
        }
    }
    tui.exit()?;

    Ok(())
}
