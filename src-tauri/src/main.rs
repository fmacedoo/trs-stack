#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rusqlite::{Connection, Result};
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug)]
struct Todo {
    id: String,
    description: String,
}

impl Serialize for Todo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut rgb = serializer.serialize_struct("Todo", 2)?;
        rgb.serialize_field("id", &self.id)?;
        rgb.serialize_field("description", &self.description)?;
        rgb.end()
    }
}

#[tauri::command]
fn connect() -> Result<Vec<Todo>, String> {
    let conn = Connection::open("todos.db").map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT * FROM todo",
    ).map_err(|e| e.to_string())?;

    let todos = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            description: row.get(1)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut result:Vec<Todo> = Vec::new();
    for row in todos {
    	result.push(row.unwrap());
    }

    // If it worked
    Ok(result)
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from auau!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
