use std::fs::{create_dir_all, read, File};
use tauri::path::BaseDirectory;

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
            .all(|v| if v == &win_dir[0] { true } else { false })
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

pub fn compute_optimal(node: &mut Node) -> Optimal<'_> {
    if node.is_leaf {
        let state_val = check_state_value(node.static_node_state);
        node.score = state_val;
        return Optimal {
            optimal: node,
            depth: node.depth,
        };
    };

    let level_pl = node.level_player;

    let optimal_child = node
        .children
        .iter_mut()
        .map(|child| compute_optimal(child))
        .enumerate()
        .reduce(|(prev_idx, prev), (curr_idx, curr)| {
            let prev_score = prev.optimal.score.unwrap();
            let curr_score = curr.optimal.score.unwrap();
            if prev_score == curr_score {
                return if prev.depth <= curr.depth {
                    (prev_idx, prev)
                } else {
                    (curr_idx, curr)
                };
            };
            match level_pl {
                PlayerType::Maximizer => {
                    if prev_score > curr_score {
                        (prev_idx, prev)
                    } else {
                        (curr_idx, curr)
                    }
                }
                _ => {
                    if prev_score < curr_score {
                        (prev_idx, prev)
                    } else {
                        (curr_idx, curr)
                    }
                }
            }
        });

    if let Some((idx, opt)) = optimal_child {
        node.score = opt.optimal.score;
        node.optimal_child = Some(idx);

        return Optimal {
            depth: opt.depth,
            optimal: node,
        };
    };
    return Optimal {
        optimal: node,
        depth: node.depth,
    };
}

pub const MAXIMIZER_BIN_FILE: &str = "maximizerTree.bin";
pub const MINIMIZER_BIN_FILE: &str = "minimizerTree.bin";

pub fn get_root_node(app: &tauri::AppHandle, player: PlayerType) -> Result<Node, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| {
            println!("Error getting app data dir: {}", e);
            "Failed to get app data dir".to_string()
        })?;

    let player_path = app_data_dir.join(match player {
        PlayerType::Maximizer => MAXIMIZER_BIN_FILE,
        PlayerType::Minimizer => MINIMIZER_BIN_FILE,
    });

    if player_path.exists() {
        let file = read(player_path).map_err(|_| "Failed to open game tree file".to_string())?;
        let node = deserialize::<Node>(&file).map_err(|e| {
            println!("Error deserialize {}", e);
            "Failed to deserialize".to_string()
        })?;

        return Ok(node);
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
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| {
            println!("Error getting app data dir {}", e);
            "Failed to get app data dir".to_string()
        })?;

    let maximizer_zip = app
        .path()
        .resolve("resources/maximizerTree.zip", BaseDirectory::Resource)
        .map_err(|e| {
            println!("Error getting zip file {}", e);
            "Failed to resolve maximizerTree.zip path".to_string()
        })?;

    let minimizer_zip = app
        .path()
        .resolve("resources/minimizerTree.zip", BaseDirectory::Resource)
        .map_err(|e| {
            println!("Error getting zip file {}", e);
            "Failed to resolve minimizerTree.zip path".to_string()
        })?;

    if !app_data_dir.exists() {
        create_dir_all(&app_data_dir)
            .map_err(|_| "Failed to create app data directory".to_string())?;
    };

    let maximizer_file = File::open(&maximizer_zip).map_err(|e| {
        println!("Error opening zip file at {:?}: {}", maximizer_zip, e);
        "Failed to open maximizer file".to_string()
    })?;

    let minimizer_file = File::open(&minimizer_zip).map_err(|e| {
        println!("Error opening zip file at {:?}: {}", minimizer_zip, e);
        "Failed to open minimizer file".to_string()
    })?;

    let mut max_zip_file = ZipArchive::new(maximizer_file).map_err(|e| {
        println!("Error creating archive struct: {}", e);
        "Failed to init zip archive".to_string()
    })?;

    let mut min_zip_file = ZipArchive::new(minimizer_file).map_err(|e| {
        println!("Error creating archive struct: {}", e);
        "Failed to init zip archive".to_string()
    })?;

    max_zip_file
        .extract(&app_data_dir)
        .map_err(|_| "Failed to extract max tree file".to_string())?;

    min_zip_file
        .extract(&app_data_dir)
        .map_err(|_| "Failed to extract min tree file".to_string())?;

    Ok(())
}
