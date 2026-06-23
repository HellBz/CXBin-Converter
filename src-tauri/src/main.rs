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

#[tauri::command]
fn open_path(path: String, open_folder: bool) -> Result<(), String> {
    let target = if open_folder {
        std::path::Path::new(&path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or(path)
    } else {
        path
    };

    let result = if cfg!(target_os = "windows") {
        if open_folder {
            std::process::Command::new("explorer").arg(&target).status()
        } else {
            std::process::Command::new("cmd")
                .args(["/c", "start", "", &target])
                .status()
        }
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("open").arg(&target).status()
    } else {
        std::process::Command::new("xdg-open").arg(&target).status()
    };

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to open path: {}", e)),
    }
}

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
            commands::supported_formats,
            open_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
