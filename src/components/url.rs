use anyhow::{Error, Ok};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
};
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
    pub fn new() -> Self {
        Self {
            text: String::new(),
            mode: Mode::Normal,
            input: Input::new(),
            action_tx: None,
        }
    }

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
                            let _ = sender
                                .send(Action::Error(format!("Failed to send action: {:?}", e)));
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
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(1)])
            .split(rect);

        let (msg, style) = match self.mode {
            Mode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "e".bold(),
                    " to start editing.".bold(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            Mode::Insert => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop editing, ".into(),
                    "Enter".bold(),
                    " to record the message".into(),
                ],
                Style::default(),
            ),
        };
        let mut text = Text::from(Line::from(msg));
        text.patch_style(style);
        let help_message = Paragraph::new(text);
        f.render_widget(help_message, chunks[0]);

        let input = Paragraph::new(self.input.input.as_str())
            .style(match self.mode {
                Mode::Normal => Style::default(),
                Mode::Insert => Style::default().fg(Color::Yellow),
            })
            .block(Block::default().borders(Borders::ALL).title("Url"));
        f.render_widget(input, chunks[1]);
        match self.mode {
            Mode::Normal => Ok(()),

            Mode::Insert => {
                f.set_cursor(
                    chunks[1].x + self.input.cursor_position as u16 + 1,
                    chunks[1].y + 1,
                );
                Ok(())
            }
        }
    }
}
