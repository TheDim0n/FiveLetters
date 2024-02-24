use crate::core::interfaces;


pub struct FiveLettersRepo {
    connection: sqlite::Connection,
}
impl FiveLettersRepo {
    pub fn new(connection: sqlite::Connection) -> FiveLettersRepo {
        FiveLettersRepo{connection}
    }
}

impl interfaces::FiveLettersRepo for FiveLettersRepo {
    fn create_tables(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.connection.execute("
        CREATE TABLE IF NOT EXISTS words (
            id     INTEGER PRIMARY KEY AUTOINCREMENT,
            value  TEXT    NOT NULL,
            status INTEGER NOT NULL
                           DEFAULT ( -1)
        );
        CREATE TABLE IF NOT EXISTS statuses (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            word_id INTEGER REFERENCES words (id)
                            NOT NULL,
            number  INTEGER NOT NULL,
            value   TEXT    NOT NULL
        );
        ")?;
        Ok(())
    }

    fn close(self) { drop(self.connection) }
}