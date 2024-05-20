// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use fiveletters::adapters::sqlite::FiveLettersRepo;
use fiveletters::core::interfaces::FiveLettersRepo as _;
use fiveletters::core::services::GameSessionService;
use fiveletters::core::entities;

use crate::structs;


#[tauri::command]
fn hello() -> String {
  String::from("Hello!")
}

#[tauri::command]
fn get_current_session(repo: &FiveLettersRepo) -> structs::GameSession {
  let connection = sqlite::open("mock/database.db").unwrap();
  let repo = FiveLettersRepo::new(connection);
  let service = GameSessionService::new(&repo);
  let session = service.get_current_session();
}


fn main() {
  let connection = sqlite::open("mock/database.db").unwrap();
  let repo = FiveLettersRepo::new(connection);
  repo.create_tables().unwrap();
  repo.fill_tables_with_init_data().unwrap_or(());
  repo.close();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      hello,
      get_current_session
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
