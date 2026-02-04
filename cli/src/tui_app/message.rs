use anyhow::{Context as _, Result};

use crate::tui_app::State;

pub(crate) enum Message {}

impl Message {
    pub(crate) fn from_event(state: &State) -> Result<Option<Message>> {
        todo!()
    }
}
