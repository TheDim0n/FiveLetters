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

    fn fill_tables_with_init_data(&self) -> Result<(), Box<dyn std::error::Error>> {
        let words_content = std::fs::read_to_string("mock/words.csv")?;
        let mut query = String::from("INSERT INTO words (id, value) VALUES");
        for row in words_content.split("\n") {
            let a: Vec<&str> = row.trim().split(';').collect();
            if a.len() == 2 {
                query.push_str(format!("\n({}, '{}'),", a[0], a[1]).as_str());
            }
        }
        match query.strip_suffix(",") {
            Some(value) => query = value.to_string(),
            None => (),
        };
        query.push(';');
        self.connection.execute(query)?;
        Ok(())
    }
}