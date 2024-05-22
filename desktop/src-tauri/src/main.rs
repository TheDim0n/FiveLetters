// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use adapters::sqlite::FiveLettersRepo;
use fiveletters::interfaces::FiveLettersRepo as _;

mod dtos;

use dtos::{GameSession, Attemption};

#[tauri::command]
fn get_actual_session() -> GameSession {
  let connection = sqlite::open("../../mock/database.db").unwrap();

  let repo = FiveLettersRepo::new(connection);
  // repo.create_tables().unwrap();
  // repo.fill_tables_with_init_data().unwrap_or(());
  let game_session = repo.get_actual_session();
  repo.close();
  GameSession{
    id: game_session.id,
    target: game_session.target,
    attemptions: game_session.attemptions.map(|x| Attemption{
      word: x.word,
      statuses: x.statuses.map(|status| status as u8)
    }),
    current_attempt: game_session.current_attempt,
    completed: game_session.completed
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_actual_session])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
