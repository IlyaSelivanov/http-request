use anyhow::Error;

use crate::ui::Event;

pub trait Component {
    fn init(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn handle_event(&mut self, event: Option<Event>) -> Result<(), Error> {
        Ok(())
    }
}
