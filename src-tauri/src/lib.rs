use std::process::{Command, Stdio};
use std::path::PathBuf;
use tauri::App;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
        
            start_backend(app);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn start_backend(app: &mut App) {
    // #[cfg(target_os = "windows")]
    // path.set_extension("exe");

    let sidecar_command = app.shell().sidecar("bin/mggraph-backend.exe").unwrap();
    let (mut rx, mut _child) = sidecar_command
    .spawn()
    .expect("Failed to spawn sidecar");

    // #[cfg(target_os = "windows")]
    // path.set_extension("exe");
}