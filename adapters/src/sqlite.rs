use core::{entities, interfaces, value_objects as vo};


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
        let query = format!("
        CREATE TABLE IF NOT EXISTS words (
            id     INTEGER PRIMARY KEY AUTOINCREMENT,
            value  TEXT    NOT NULL,
            status INTEGER NOT NULL
                           DEFAULT ( {default_status})
        );
        CREATE TABLE IF NOT EXISTS statuses (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            word_id INTEGER REFERENCES words (id)
                            NOT NULL,
            number  INTEGER NOT NULL,
            value   TEXT    NOT NULL
        );
        ", default_status=vo::WordStatuses::Inactive);
        self.connection.execute(query)?;
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

    fn set_next_solution(&self) {
        let mut query = format!(
            "update words set status = {new_status} where status = {old_status}",
            new_status=vo::WordStatuses::Solved,
            old_status=vo::WordStatuses::Active
        );
        self.connection.execute(query).unwrap();

        query = format!("
        update words set status = {new_status} where id = (
            select
                id
            from words
            where status = {old_status}
            order by random()
            limit 1
        )
        ", old_status=vo::WordStatuses::Inactive, new_status=vo::WordStatuses::Active);
        self.connection.execute(query).unwrap();
    }

    fn get_actual_session(&self) -> entities::GameSession {
        let query = format!("
        select
            id,
            value
        from words
        where status = {status}
        limit 1
        ", status=vo::WordStatuses::Active);
        let mut current_attempt_res = self.connection
            .prepare(query)
            .unwrap()
            .into_iter()
            .map(|row| row.unwrap());
        let current_attempt = current_attempt_res.next();
        match current_attempt {
            Some(row) => {
                let id = row.read::<i64, _>("id") as usize;
                let word: &str = row.read::<&str, _>("value");
                let mut game_session = entities::GameSession::from((id, word));

                let query = "
                SELECT
                    *
                from statuses
                WHERE word_id = :word_id
                ";
                for row in self.connection
                    .prepare(query)
                    .unwrap()
                    .into_iter()
                    .bind((1, game_session.id as i64))
                    .unwrap()
                    .map(|row| row.unwrap()) {
                        game_session.add_attemption(
                            &row.read::<&str, _>("value").to_owned(),
                            row.read::<i64, _>("number") as usize
                        )
                    }
                return game_session;
            }
            None => {
                self.set_next_solution();
                self.get_actual_session()
            }
        }
    }

    fn add_attemption(&self, word_id: usize, number: usize, value: String) -> Result<(), Box<dyn std::error::Error>> {
        let query = "
        insert INTO statuses
        (word_id, number, value)
        values
        (:word_id, :number, :value)
        ";
        let stmt = self.connection.prepare(query).unwrap().into_iter();
        stmt.bind::<&[(_, sqlite::Value)]>(&[
            (":word_id", (word_id as i64).into()),
            (":number", (number as i64).into()),
            (":value", value.into())
        ])?.next();
        Ok(())
    }

}