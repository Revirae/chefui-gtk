mod cuisine;

use rusqlite::Connection;

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
