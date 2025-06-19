use tauri::App;
use tauri_plugin_shell::ShellExt;
use std::sync::{Arc, Mutex};
use tauri_plugin_shell::process::CommandChild;
use std::path::{PathBuf};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let sidecar_process: Arc<Mutex<Option<CommandChild>>> = Arc::new(Mutex::new(None));
    let sidecar_setup = Arc::clone(&sidecar_process);
    let sidecar_exit = Arc::clone(&sidecar_process);

    let mut backend_path = PathBuf::from("bin/mggraph-backend");
    #[cfg(target_os = "windows")]
    backend_path.set_extension("exe");

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
        
            start_backend(backend_path, app, &sidecar_setup);

            Ok(())
        })
        .on_window_event(move |window, event| match event {
            tauri::WindowEvent::CloseRequested { .. } => {
                exit_backend(&sidecar_exit);
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}

#[tauri::command]
fn start_backend(backend_path: PathBuf, app: &mut App, sidecar_process: &Arc<Mutex<Option<CommandChild>>>) {

    // #[cfg(target_os = "windows")]
    // path.set_extension("exe");

    let sidecar_command = app.shell().sidecar(backend_path).unwrap();
    let (mut rx, mut _child) = sidecar_command
    .spawn()
    .expect("Failed to spawn sidecar");

    *sidecar_process.lock().unwrap() = Some(_child);
}

#[tauri::command]
fn exit_backend(sidecar_process: &Arc<Mutex<Option<CommandChild>>>) {
    if let Some(child) = sidecar_process.lock().unwrap().take() {
        if let Err(e) = child.kill() {
            eprintln!("{}", e);
        }
    }
}