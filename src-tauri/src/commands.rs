use std::sync::Mutex;

use tauri::{Manager, State};

use crate::{
    minimax::{get_node_from_state, make_move},
    utils::{MAXIMIZER_BIN_FILE, MINIMIZER_BIN_FILE, decompress_zip_files, get_root_node},
    val_types::{BoardFieldVal, Node, PlayerType},
};

#[tauri::command]
pub async fn play_move<'a>(state: State<'a, Mutex<Node>>) -> Result<BoardFieldVal, String> {
    let mut node = state
        .lock()
        .map_err(|e| {
            println!("Error reading state {}", e);
            "Failed to get app state".to_string()
        })?;
    let nxt_move = make_move(&mut node);
    Ok(nxt_move)
}

#[tauri::command]
pub async fn set_node<'a>(
    state: State<'a, Mutex<Node>>,
    board: [[Option<PlayerType>; 3]; 3],
) -> Result<String, String> {
    let mut node = state
        .lock()
        .map_err(|e| {
            println!("Error reading state {}", e);
            "Failed to get app state".to_string()
        })?;
    let new_node = get_node_from_state(board, &mut node);
    if let Some(new) = new_node {
        *node = new;
        return Ok("Node set".to_string());
    };
    Ok("Node not set".to_string())
}

#[tauri::command]
pub async fn reset_bot<'a>(state: State<'a, Mutex<Node>>, app: tauri::AppHandle, player: PlayerType) -> Result<(), String> {
    let max_file = app.path().app_data_dir().map_err(|e|{
        println!("Error read app data dir {}", e);
        "Failed to get app data dir".to_string()
    })?.join(MAXIMIZER_BIN_FILE);

    let min_file = app.path().app_data_dir().map_err(|e|{
        println!("Error read app data dir {}", e);
        "Failed to get app data dir".to_string()
    })?.join(MINIMIZER_BIN_FILE);

    if !max_file.exists() || !min_file.exists() {
        println!("Decompressing zip file...");
        decompress_zip_files(&app)?;
        println!("Decompression finished.");
    };

    let n = get_root_node(&app, player)?;
    dbg!(&n.static_node_state);
    let mut node = state
        .lock()
        .map_err(|e| {
            println!("Error reading state {}", e);
            "Failed to get app state".to_string()
        })?;
    *node = n;
    Ok(())
}
