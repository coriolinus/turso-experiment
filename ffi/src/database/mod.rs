use crate::{Context as _, Result};
use turso::Connection;
use wasm_bindgen::prelude::*;

/// A connection to a Turso database
#[wasm_bindgen]
pub struct Database {
    pub(crate) connection: Connection,
}

#[wasm_bindgen]
impl Database {
    /// Connect to a database
    pub async fn connect(name: &str) -> Result<Self> {
        let database = turso::Builder::new_local(name)
            .build()
            .await
            .context("building database")?;
        let connection = database.connect().context("connecting to database")?;
        Ok(Self { connection })
    }
}
