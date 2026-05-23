use std::sync::Mutex;

use crate::utils::get_root_node;

pub mod commands;
pub mod minimax;
pub mod utils;
pub mod val_types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(get_root_node()))
        .invoke_handler(tauri::generate_handler![
            commands::play_move,
            commands::set_node,
            commands::reset_bot
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
