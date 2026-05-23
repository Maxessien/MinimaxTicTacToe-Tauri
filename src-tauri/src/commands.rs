use std::sync::Mutex;

use tauri::Manager;

use crate::{
    minimax::{get_node_from_state, make_move},
    utils::get_root_node,
    val_types::{BoardFieldVal, Node, PlayerType},
};

#[tauri::command]
pub async fn play_move(app: tauri::AppHandle) -> Result<BoardFieldVal, String> {
    let state = app.state::<Mutex<Node>>();
    let mut node = state
        .lock()
        .map_err(|_| "Failed to get app state".to_string())?;
    Ok(make_move(&mut node))
}

#[tauri::command]
pub async fn set_node(
    app: tauri::AppHandle,
    board: [[Option<PlayerType>; 3]; 3],
) -> Result<(), String> {
    let state = app.state::<Mutex<Node>>();
    let mut node = state
        .lock()
        .map_err(|_| "Failed to get app state".to_string())?;
    let new_node = get_node_from_state(board, &mut node);
    if let Some(new) = new_node {
        *node = new
    };
    Ok(())
}

#[tauri::command]
pub async fn reset_bot(app: tauri::AppHandle) -> Result<(), String> {
    let state = app.state::<Mutex<Node>>();
    let mut node = state
        .lock()
        .map_err(|_| "Failed to get app state".to_string())?;
    *node = get_root_node();
    Ok(())
}
