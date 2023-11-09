use anyhow::Error;
use crossterm::event::KeyCode;
use ratatui::prelude::Rect;
use tokio::sync::mpsc;

use crate::{
    action::Action,
    components::{url::Url, *},
    tui,
};

/// A struct representing the application.
/// This struct contains the main logic for the application, including handling user input and
/// displaying output to the user.
pub struct App {
    pub messages: Vec<String>,
    pub should_quit: bool,
    pub components: Vec<Box<dyn Component>>,
}

impl App {
    pub fn new() -> App {
        let url = Url::new();
        App {
            messages: Vec::new(),
            should_quit: false,
            components: vec![Box::new(url)],
        }
    }

    // pub async fn submit_message(&mut self) {
    //     self.messages.push(self.input.clone());

    //     let request = HttpRequest::new(crate::http_client::HttpMethod::Get, self.input.as_str());
    //     let response = request.send().await;

    //     self.messages.push(response.status_code.to_string());

    //     self.input.clear();
    //     self.reset_cursor();
    // }

    pub async fn run(&mut self) -> Result<(), Error> {
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        let mut tui = tui::Tui::new()?;
        // tui.tick_rate(self.tick_rate);
        // tui.frame_rate(self.frame_rate);
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(action_tx.clone())?;
        }

        // for component in self.components.iter_mut() {
        //     component.register_config_handler(self.config.clone())?;
        // }

        for component in self.components.iter_mut() {
            component.init()?;
        }

        loop {
            let e = tui.next().await?;
            match e {
                tui::Event::Quit => action_tx.send(Action::Quit)?,
                tui::Event::Tick => action_tx.send(Action::Tick)?,
                tui::Event::Render => action_tx.send(Action::Render)?,
                tui::Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
                tui::Event::Key(key) => match key.code {
                    KeyCode::Char('e') => action_tx.send(Action::EnterUrlInsert)?,
                    KeyCode::Esc => action_tx.send(Action::EnterNormal)?,
                    KeyCode::Char('q') => action_tx.send(Action::Quit)?,
                    _ => {}
                },
                _ => {}
            }
            for component in self.components.iter_mut() {
                if let Some(action) = component.handle_events(Some(e.clone()))? {
                    action_tx.send(action)?;
                }
            }

            while let Ok(action) = action_rx.try_recv() {
                match action {
                    Action::Quit => self.should_quit = true,
                    Action::Resize(w, h) => {
                        tui.resize(Rect::new(0, 0, w, h))?;
                        tui.draw(|f| {
                            for component in self.components.iter_mut() {
                                let r = component.render(f, f.size());
                                if let Err(e) = r {
                                    action_tx
                                        .send(Action::Error(format!("Failed to draw: {:?}", e)))
                                        .unwrap();
                                }
                            }
                        })?;
                    }
                    Action::Render => {
                        tui.draw(|f| {
                            for component in self.components.iter_mut() {
                                let r = component.render(f, f.size());
                                if let Err(e) = r {
                                    action_tx
                                        .send(Action::Error(format!("Failed to draw: {:?}", e)))
                                        .unwrap();
                                }
                            }
                        })?;
                    }
                    _ => {}
                }

                for component in self.components.iter_mut() {
                    if let Some(action) = component.update(action.clone())? {
                        action_tx.send(action)?
                    };
                }
            }
            if self.should_quit {
                break;
            }
        }

        tui.exit()?;
        Ok(())
    }
}
