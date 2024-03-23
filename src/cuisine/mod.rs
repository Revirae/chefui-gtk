mod cuisine;
// mod action;

// use std::iter::Map

// use std::collections::HashMap;

use rusqlite::Connection;

// use crate::action::Action;

#[derive(Debug, Clone)]
pub struct Store {
    pub filename: String,
}

impl Store {
    fn link(
        &self,
    ) -> Result<Connection, rusqlite::Error> {
        Connection::open(self.filename.clone())
    }
}
