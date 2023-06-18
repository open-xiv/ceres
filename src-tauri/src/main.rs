#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::Manager;

mod config_manager;
mod export;
mod logs_reader;
mod meta_loader;
mod model;

fn main() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // init meta
            meta_loader::load_meta,
            // config manager
            config_manager::read_config,
            config_manager::write_config,
            // logs analysis
            logs_reader::load_logs,
            // export tools
            export::to_notion,
            export::to_json,
            export::count_times,
        ])
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                let window = _app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
