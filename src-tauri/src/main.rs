// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod authenticate;
use authenticate::{get_user, save_user};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_user, save_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
