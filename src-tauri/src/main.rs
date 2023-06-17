#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Manager, Menu, Submenu};
use tauri::api::shell;

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
        ])
        .menu(tauri::Menu::os_default("Ceres").add_submenu(Submenu::new(
            "Help",
            Menu::with_items([
                CustomMenuItem::new("Online Documentation", "Online Documentation").into(),
            ]),
        )))
        .on_menu_event(|event| {
            let event_name = event.menu_item_id();
            match event_name {
                "Online Documentation" => {
                    let url = "https://github.com/open-xiv/ceres".to_string();
                    shell::open(&event.window().shell_scope(), url, None).unwrap();
                }
                _ => {}
            }
        })
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
