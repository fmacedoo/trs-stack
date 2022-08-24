#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod database;
mod models;

use database::connect;
use models::todo::Todo;

#[tauri::command]
fn query_todos() -> Result<Vec<Todo>, String> {
    connect()
        .map_err(|err| err.to_string())?
        .todos()
        .query_all()
        .map_err(|err| err.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![query_todos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
