use crate::{
    utils::compute_optimal,
    val_types::{BoardFieldVal, Node, PlayerType},
};

pub fn get_node_from_state(board: [[Option<PlayerType>; 3]; 3], node: &mut Node) -> Option<Node> {
    let flattened: Vec<&Option<PlayerType>> = board.iter().flatten().collect();
    let match_child = node.children.iter().find(|n| {
        n.static_node_state
            .iter()
            .flatten()
            .enumerate()
            .all(|(idx, v)| v == flattened[idx])
    });
    Some(match_child?.clone())
}

pub fn make_move(node: &mut Node) -> BoardFieldVal {
    compute_optimal(node);
    let mut val = BoardFieldVal { col: 1, row: 1 };
    if let Some(n) = node.optimal_child.take() {
        let next_flat = node.children[n].static_node_state;
        for (idx, node_val) in node.static_node_state.iter().enumerate() {
            for (inn_idx, inn_node) in node_val.iter().enumerate() {
                if let None = inn_node {
                    if let Some(_) = &next_flat[idx][inn_idx] {
                        val.col = inn_idx as u32;
                        val.row = idx as u32;
                    }
                }
            }
        }
    };
    val
}
