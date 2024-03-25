#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use once_cell::sync::Lazy;
use reqwest::Client;
use tauri::Manager;

mod apis;
mod export;
mod model;
mod parser;
mod tools;

pub static GLOBAL_CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

fn main() {
    let ctx = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // init meta
            parser::meta::load_meta,
            // config manager
            parser::config::read_config,
            parser::config::write_config,
            // logs analysis
            parser::actlog::load_act_log,
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
