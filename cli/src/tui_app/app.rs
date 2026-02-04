use std::path::Path;

use anyhow::{Context as _, Result, anyhow};
use glob::glob;
use ratatui::Frame;
use turso::Connection;

use crate::tui_app::{Message, State};

#[derive(Debug)]
pub(crate) struct App {
    pub(crate) connection: Connection,
    pub(crate) state: State,
}

impl App {
    pub(crate) async fn new(db_path: impl AsRef<Path>) -> Result<Self> {
        let db_path = std::path::absolute(db_path).context("absolutizing path")?;

        let db_exists = std::fs::exists(&db_path).context("checking for db path existence")?;

        // ensure parent path exists
        let parent = db_path
            .parent()
            .ok_or(anyhow!("cannot use `/` as the db"))?;
        std::fs::create_dir_all(parent).context("creating db parent dir")?;

        let db_path = db_path
            .to_str()
            .context("db_path could not be represented as unicode")?;
        let database = turso::Builder::new_local(db_path)
            .build()
            .await
            .context("building database")?;
        let mut connection = database.connect().context("connecting to database")?;

        if !db_exists {
            todo_list::apply_schema(&mut connection)
                .await
                .context("applying schema to new database file")
                .inspect_err(|_err| {
                    // best effort
                    // first the db itself
                    let _ = std::fs::remove_file(db_path);
                    // then ancillary files by glob if necessary
                    if let Some(paths) = glob(&format!("{db_path}*")).ok() {
                        for path in paths {
                            if let Some(path) = path.ok() {
                                let _ = std::fs::remove_file(path);
                            }
                        }
                    }
                })?;
        }

        Ok(Self {
            connection,
            state: State::Initial,
        })
    }

    /// Process an incoming message, updating the app state appropriately.
    pub(crate) async fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Quit => {
                self.state = State::Exit;
                None
            }
        }
    }

    /// Render the TUI according to the current state
    pub(crate) fn view(&self, frame: &mut Frame) {
        match &self.state {
            State::Initial => {
                frame.render_widget("spinning up (<q> or <esc> to quit)", frame.area())
            }
            State::ListSelect(hash_map) => todo!(),
            State::Exit => unreachable!("app should always exit prior to rendering this"),
        }
    }
}
