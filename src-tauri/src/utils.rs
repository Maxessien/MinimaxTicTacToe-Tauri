use crate::val_types::{BoardValues, Node, Optimal, PlayerType};

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

pub fn check_state_value(board: [[Option<PlayerType>; 3]; 3]) -> BoardValues {
    let all_wins = get_possible_outcomes(board);
    for win_dir in all_wins {
        if win_dir
            .iter()
            .all(|v| if let Some(_) = v { true } else { false })
        {
            match win_dir[0] {
                Some(val) => match val {
                    PlayerType::Maximizer => return BoardValues::High,
                    PlayerType::Minimizer => return BoardValues::Low,
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
        return BoardValues::Draw;
    };
    BoardValues::Null
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
            let prev_score = prev.optimal.score as u32;
            let curr_score = curr.optimal.score as u32;
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
