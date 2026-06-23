// Prevents additional console window on Windows in release mode.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;
mod commands;
mod cxbin;
mod export;

fn main() {
    cli::try_cli_mode();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::convert_cxbin,
            commands::supported_formats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
