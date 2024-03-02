use fiveletters::adapters::sqlite::FiveLettersRepo;
use fiveletters::core::interfaces::FiveLettersRepo as _;

fn main() {
    let connection = sqlite::open("mock/database.db").unwrap();

    let repo = FiveLettersRepo::new(connection);
    // repo.create_tables().unwrap();
    // repo.fill_tables_with_init_data().unwrap();
    repo.get_actual_session();
    repo.close();
}