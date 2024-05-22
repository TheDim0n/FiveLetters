use adapters::sqlite::FiveLettersRepo;
use core::interfaces::FiveLettersRepo as _;

fn main() {
    let connection = sqlite::open("mock/database.db").unwrap();

    let repo = FiveLettersRepo::new(connection);
    repo.create_tables().unwrap();
    repo.fill_tables_with_init_data().unwrap_or(());
    repo.add_attemption(3015, 1, String::from("салон")).unwrap();
    repo.add_attemption(3015, 2, String::from("фасон")).unwrap();
    let game_session = repo.get_actual_session();
    println!("{:?}", game_session);
    repo.close();
}
