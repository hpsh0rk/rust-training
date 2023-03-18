// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn example_sql() -> String {
  queryer::example_sql()
}

#[tauri::command]
async fn query(sql: String) -> Result<String, String> {
  let mut data = queryer::query(&sql).await.map_err(|err| err.to_string())?;
  Ok(data.to_csv().map_err(|err| err.to_string())?)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, example_sql, query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
