use anyhow::Error;
use crossterm::event::KeyCode;
use ratatui::{prelude::Rect, widgets::ListState};
use tokio::sync::mpsc;

use crate::{
    action::Action,
    components::{url::Url, *},
    tui,
};

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

    #[allow(dead_code)]
    fn unselect(&mut self) {
        self.state.select(None);
    }
}

/// A struct representing the application.
/// This struct contains the main logic for the application, including handling user input and
/// displaying output to the user.
pub struct App {
    pub messages: Vec<String>,
    pub should_quit: bool,
    pub methods: StatefulList<String>,
    pub components: Vec<Box<dyn Component>>,
}

impl App {
    pub fn new() -> App {
        let url = Url::new();
        App {
            messages: Vec::new(),
            should_quit: false,
            methods: StatefulList::with_items(vec![
                String::from("GET"),
                String::from("POST"),
                String::from("PUT"),
                String::from("DELETE"),
            ]),
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

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn select_next_method(&mut self) {
        self.methods.next();
    }

    pub fn select_previous_method(&mut self) {
        self.methods.previous();
    }

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
            if let e = tui.next().await? {
                match e {
                    tui::Event::Quit => action_tx.send(Action::Quit)?,
                    tui::Event::Tick => action_tx.send(Action::Tick)?,
                    tui::Event::Render => action_tx.send(Action::Render)?,
                    tui::Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
                    tui::Event::Key(key) => match key.code {
                        KeyCode::Enter => action_tx.send(Action::EnterUrlInsert)?,
                        KeyCode::Esc => action_tx.send(Action::EnterNormal)?,
                        _ => {}
                    },
                    _ => {}
                }
                for component in self.components.iter_mut() {
                    if let Some(action) = component.handle_events(Some(e.clone()))? {
                        action_tx.send(action)?;
                    }
                }
            }

            while let Ok(action) = action_rx.try_recv() {
                // if action != Action::Tick && action != Action::Render {
                //     log::debug!("{action:?}");
                // }
                match action {
                    // Action::Tick => {
                    //     self.last_tick_key_events.drain(..);
                    // }
                    Action::Quit => self.should_quit = true,
                    // Action::Suspend => self.should_suspend = true,
                    // Action::Resume => self.should_suspend = false,
                    Action::Resize(w, h) => {
                        tui.resize(Rect::new(0, 0, w, h))?;
                        tui.draw(|f| {
                            for component in self.components.iter_mut() {
                                let r = component.render(f, f.size());
                                // if let Err(e) = r {
                                //     action_tx
                                //         .send(Action::Error(format!("Failed to draw: {:?}", e)))
                                //         .unwrap();
                                // }
                            }
                        })?;
                    }
                    Action::Render => {
                        tui.draw(|f| {
                            for component in self.components.iter_mut() {
                                let r = component.render(f, f.size());
                                // if let Err(e) = r {
                                //     action_tx
                                //         .send(Action::Error(format!("Failed to draw: {:?}", e)))
                                //         .unwrap();
                                // }
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
            // if self.should_suspend {
            //     tui.suspend()?;
            //     action_tx.send(Action::Resume)?;
            //     tui = tui::Tui::new()?;
            //     tui.tick_rate(self.tick_rate);
            //     tui.frame_rate(self.frame_rate);
            //     tui.enter()?;
            // } else if self.should_quit {
            //     tui.stop()?;
            //     break;
            // }
        }
        tui.exit()?;
        Ok(())
    }
}
