use adapters::sqlite::FiveLettersRepo;
use core::interfaces::FiveLettersRepo as _;
use core::services::GameSessionService;

fn main() {
    let connection = sqlite::open("mock/database.db").unwrap();

    let repo = FiveLettersRepo::new(connection);
    repo.create_tables().unwrap();
    repo.fill_tables_with_init_data().unwrap_or(());
    let service = GameSessionService::new(&repo);
    let session = service.get_current_session();
    println!("{:?}", session);
    repo.close();
}
