// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod authenticate;
use authenticate::{get_user, save_user};
use std::process::{Command, Output};

#[tauri::command]
fn stockfish(path: &str) -> Vec<String> {
    let output: Output;
    #[cfg(target_os = "windows")]
    {
        output = Command::new(path.to_owned() + "./stockfish.exe")
            .arg("bench")
            .output()
            .expect("Failed to execute the command.");
    }
    #[cfg(not(target_os = "windows"))]
    {
        output = Command::new(path.to_owned() + "./stockfish")
            .arg("bench")
            .output()
            .expect("Failed to execute the command.");
    }

    let combined_output = [&output.stdout[..], &output.stderr[..]].concat();
    let output_string = String::from_utf8_lossy(&combined_output);
    let lines: Vec<String> = output_string.lines().map(|s| s.to_string()).collect();

    lines
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_user, save_user, stockfish])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
