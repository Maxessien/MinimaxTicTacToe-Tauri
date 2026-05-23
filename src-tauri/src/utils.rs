use std::{fs::{File, create_dir_all, read}};

use tauri::Manager;
use wincode::deserialize;
use zip::ZipArchive;

use crate::val_types::{Node, Optimal, PlayerType};

pub fn get_empty_board() -> [[Option<PlayerType>; 3]; 3] {
    [[None, None, None], [None, None, None], [None, None, None]]
}

pub fn get_possible_outcomes(board: [[Option<PlayerType>; 3]; 3]) -> [[Option<PlayerType>; 3]; 8] {
    [
        board[0],
        board[1],
        board[2],
        [board[0][0], board[1][0], board[2][0]],
        [board[0][1], board[1][1], board[2][1]],
        [board[0][2], board[1][2], board[2][2]],
        [board[0][0], board[1][1], board[2][2]],
        [board[0][2], board[1][1], board[2][0]],
    ]
}

pub fn check_state_value(board: [[Option<PlayerType>; 3]; 3]) -> Option<i32> {
    let all_wins = get_possible_outcomes(board);
    for win_dir in all_wins {
        if win_dir
            .iter()
            .all(|v| if let Some(_) = v { true } else { false })
        {
            match win_dir[0] {
                Some(val) => match val {
                    PlayerType::Maximizer => return Some(1),
                    PlayerType::Minimizer => return Some(-1),
                },
                None => continue,
            };
        };
    }
    if all_wins
        .iter()
        .flatten()
        .all(|v| if let Some(_) = v { true } else { false })
    {
        return Some(0);
    };
    None
}

pub fn compute_optimal<'a>(node: &mut Node) -> Optimal {
    if node.is_leaf {
        let state_val = check_state_value(node.static_node_state);
        node.score = state_val;
        return Optimal {
            optimal: node.clone(),
            depth: node.depth,
        };
    };

    let optimal_child = node
        .children
        .iter_mut()
        .map(|v| compute_optimal(v))
        .reduce(|prev, curr| {
            let prev_score = prev.optimal.score;
            let curr_score = curr.optimal.score;
            let level_pl = node.level_player;
            if prev_score == curr_score {
                return if prev.depth > curr.depth { curr } else { prev };
            };
            match level_pl {
                PlayerType::Maximizer => {
                    if prev_score > curr_score {
                        prev
                    } else {
                        curr
                    }
                }
                _ => {
                    if prev_score > curr_score {
                        curr
                    } else {
                        prev
                    }
                }
            }
        })
        .unwrap();

    node.score = optimal_child.optimal.score;
    node.optimal_child = Some(Box::new(optimal_child.optimal.clone()));

    return optimal_child;
}

const MAXIMIZER_BIN_FILE: &str = "maximizerTree.bin";
const MINIMIZER_BIN_FILE: &str = "minimizerTree.bin";

pub fn get_root_node(app: &tauri::AppHandle, player: PlayerType) -> Result<Node, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|_| "Failed to get app data dir".to_string())?;

    let player_path = app_data_dir.join(match player {
        PlayerType::Maximizer => MAXIMIZER_BIN_FILE,
        PlayerType::Minimizer => MINIMIZER_BIN_FILE,
    });

    if player_path.exists() {
        let file = read(player_path).map_err(|_| "Failed to open game tree file".to_string())?;
        let node = deserialize::<Node>(&file).map_err(|e|{
            println!("Error deserialize {}", e);
            "Failed to deserialize".to_string()
        })?;

        return  Ok(node);
    };

    Ok(Node {
        children: Vec::new(),
        depth: 0,
        is_leaf: false,
        is_root: true,
        level_player: PlayerType::Maximizer,
        optimal_child: None,
        score: None,
        static_node_state: get_empty_board(),
    })
}

pub fn decompress_zip_files(app: &tauri::AppHandle) -> Result<(), String> {
    let path = app
        .path()
        .resource_dir()
        .map_err(|_| "Failed to resolve resources dir".to_string())?;    

    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|_| "Failed to get app data dir".to_string())?;

    let maximizer_zip = path.join("maximizerTree.zip");
    let minimizer_zip = path.join("minimizerTree.zip");

    if !app_data_dir.exists() {
        create_dir_all(&app_data_dir).map_err(|_| "Failed to create app data directory".to_string())?;
    };

    let maximizer_file =
        File::open(maximizer_zip).map_err(|_| "Failed to open maximizer file".to_string())?;
    let minimizer_file =
        File::open(minimizer_zip).map_err(|_| "Failed to open minimizer file".to_string())?;

    let mut max_zip_file =
        ZipArchive::new(maximizer_file).map_err(|_| "Failed to init zip archive".to_string())?;
    let mut min_zip_file =
        ZipArchive::new(minimizer_file).map_err(|_| "Failed to init zip archive".to_string())?;

    max_zip_file
        .extract(&app_data_dir)
        .map_err(|_| "Failed to extract max tree file".to_string())?;
    min_zip_file
        .extract(&app_data_dir)
        .map_err(|_| "Failed to extract min tree file".to_string())?;
    Ok(())
}
