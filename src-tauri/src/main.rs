// Prevents an additional console window in GUI release mode.
// CLI mode re-attaches the parent console so output is still visible.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;
mod commands;
mod cxbin;
mod export;

#[cfg(windows)]
fn attach_parent_console() {
    use windows::Win32::System::Console::AttachConsole;
    use windows::Win32::System::Console::ATTACH_PARENT_PROCESS;
    unsafe {
        let _ = AttachConsole(ATTACH_PARENT_PROCESS);
    }
}

#[cfg(not(windows))]
fn attach_parent_console() {}

fn main() {
    // In CLI mode, re-attach the parent console so output is visible in the terminal.
    if std::env::args().len() > 1 {
        attach_parent_console();
    }

    cli::try_cli_mode();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::convert_cxbin,
            commands::get_geometry,
            commands::supported_formats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
