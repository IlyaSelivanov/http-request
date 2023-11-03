use anyhow::Error;
use crossterm::event::KeyEvent;
use futures::{FutureExt, StreamExt};
use tokio::{sync::mpsc, task::JoinHandle};

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Error,
    Tick,
    Key(KeyEvent),
}

pub struct EventHandler {
    _tx: mpsc::UnboundedSender<Event>,
    rx: mpsc::UnboundedReceiver<Event>,
    #[warn(dead_code)]
    task: Option<JoinHandle<()>>,
}

impl EventHandler {
    pub fn new() -> Self {
        let tick_rate = std::time::Duration::from_millis(250);

        let (tx, rx) = mpsc::unbounded_channel();
        let _tx = tx.clone();

        let task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut interval = tokio::time::interval(tick_rate);
            loop {
                let delay = interval.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                  maybe_event = crossterm_event => {
                    match maybe_event {
                      Some(Ok(evt)) => {
                        match evt {
                            crossterm::event::Event::Key(key) => {
                                if key.kind==crossterm::event::KeyEventKind::Press {
                                    tx.send(Event::Key(key)).unwrap();
                                }
                            },
                            crossterm::event::Event::FocusGained => {},
                            crossterm::event::Event::FocusLost => {},
                            crossterm::event::Event::Mouse(_) => {},
                            crossterm::event::Event::Paste(_) => {},
                            crossterm::event::Event::Resize(_, _) => {},
                        }
                      }
                      Some(Err(_)) => {
                        tx.send(Event::Error).unwrap();
                      }
                      None => {},
                    }
                  },
                  _ = delay => {
                      tx.send(Event::Tick).unwrap();
                  },
                }
            }
        });

        Self {
            _tx,
            rx,
            task: Some(task),
        }
    }

    pub async fn next(&mut self) -> Result<Event, Error> {
        self.rx
            .recv()
            .await
            .ok_or(anyhow::anyhow!("Unable to get event"))
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
