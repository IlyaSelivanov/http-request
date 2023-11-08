use anyhow::Error;
use crossterm::event::{KeyCode, KeyEvent};
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, input::Input};

use super::Component;

pub enum Mode {
    Normal,
    Insert,
}

pub struct Url {
    pub text: String,
    pub mode: Mode,
    pub input: Input,
    pub action_tx: Option<UnboundedSender<Action>>,
}

impl Url {
    pub fn add(&mut self, s: String) {
        self.text = s;
    }
}

impl Component for Url {
    fn register_action_handler(
        &mut self,
        tx: UnboundedSender<Action>,
    ) -> Result<(), anyhow::Error> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn init(&mut self) -> Result<(), anyhow::Error> {
        self.input.reset_cursor();
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, Error> {
        let action = match self.mode {
            Mode::Normal => return Ok(None),
            Mode::Insert => match key.code {
                KeyCode::Esc => Action::EnterNormal,
                KeyCode::Enter => {
                    if let Some(sender) = &self.action_tx {
                        if let Err(e) =
                            sender.send(Action::CompleteInput(self.input.value().to_string()))
                        {
                            anyhow::anyhow!("Failed to send action: {:?}", e);
                        }
                    }
                    Action::EnterNormal
                }
                _ => {
                    self.input.handle_event(&crossterm::event::Event::Key(key));
                    Action::Update
                }
            },
        };
        Ok(Some(action))
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>, Error> {
        match action {
            Action::CompleteInput(s) => self.add(s),
            Action::EnterNormal => {
                self.mode = Mode::Normal;
            }
            Action::EnterUrlInsert => {
                self.mode = Mode::Insert;
            }
            _ => (),
        }
        Ok(None)
    }

    fn render(
        &mut self,
        f: &mut ratatui::Frame<'_>,
        rect: ratatui::prelude::Rect,
    ) -> Result<(), anyhow::Error> {
        todo!()
    }
}
