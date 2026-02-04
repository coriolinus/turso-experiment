use std::time::Duration;

use anyhow::{Context as _, Result};
use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::tui_app::State;

pub(crate) enum Message {
    Quit,
}

impl Message {
    pub(crate) fn from_event(_state: &State) -> Result<Option<Message>> {
        if !event::poll(Duration::from_millis(100)).context("polling for event")? {
            return Ok(None);
        }
        let Event::Key(key_event) = event::read().context("reading event")? else {
            return Ok(None);
        };
        if matches!(key_event.code, KeyCode::Esc | KeyCode::Char('q')) {
            return Ok(Some(Message::Quit));
        }
        Ok(None)
    }
}
