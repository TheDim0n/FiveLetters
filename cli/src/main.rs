use adapters::sqlite::FiveLettersRepo;
use core::interfaces::FiveLettersRepo as _;

fn main() {
    let connection = sqlite::open("mock/database.db").unwrap();

    let repo = FiveLettersRepo::new(connection);
    repo.create_tables().unwrap();
    repo.fill_tables_with_init_data().unwrap_or(());

    println!("{}", repo.is_word_exists(&"унция".to_string()));

    repo.close();
}
