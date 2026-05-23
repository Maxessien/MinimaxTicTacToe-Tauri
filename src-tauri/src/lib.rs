use std::sync::Mutex;

use crate::utils::get_empty_board;
use crate::val_types::{Node, PlayerType};

pub mod commands;
pub mod minimax;
pub mod utils;
pub mod val_types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(Node {
        children: Vec::new(),
        depth: 0,
        is_leaf: false,
        is_root: true,
        level_player: PlayerType::Maximizer,
        optimal_child: None,
        score: None,
        static_node_state: get_empty_board(),
    }))
        .invoke_handler(tauri::generate_handler![
            commands::play_move,
            commands::set_node,
            commands::reset_bot
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
