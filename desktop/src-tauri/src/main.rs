// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use adapters::sqlite::FiveLettersRepo;
use fiveletters::interfaces::FiveLettersRepo as _;

mod dtos;

use dtos::{GameSession, Attemption};
use tauri::Manager;

struct DBFilePath(std::path::PathBuf);

#[tauri::command]
fn get_actual_session(db_path: tauri::State<DBFilePath>) -> GameSession {
  
  let connection = sqlite::open(&db_path.0).unwrap();

  let repo = FiveLettersRepo::new(connection);
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


#[tauri::command]
fn save_attemption(
  word_id: usize,
  number: usize,
  value: String,
  db_path: tauri::State<DBFilePath>
) -> Result<(), ()> {
  
  let connection = sqlite::open(&db_path.0).unwrap(); 

  let repo = FiveLettersRepo::new(connection);
  match repo.is_word_exists(&value) {
      true => repo.add_attemption(word_id, number, value).unwrap(),
      false => {
        repo.close();
        return Err(())
      }
  }
  
  repo.close();
  Ok(())
}

#[tauri::command]
fn set_next_session(db_path: tauri::State<DBFilePath>) {
  let connection = sqlite::open(&db_path.0).unwrap();
  let repo = FiveLettersRepo::new(connection);
  repo.set_next_solution();
  repo.close();
}


fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let handle = app.handle();
      let db_path = handle.path_resolver()
        .resolve_resource("./mock/database.db")
        .expect("failed to resolve resource");
      app.manage(DBFilePath(db_path));
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      get_actual_session,
      save_attemption,
      set_next_session
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
