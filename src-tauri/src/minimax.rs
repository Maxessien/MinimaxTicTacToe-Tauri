use crate::{
    utils::compute_optimal,
    val_types::{BoardFieldVal, Node, PlayerType},
};

pub fn get_node_from_state(board: [[Option<PlayerType>; 3]; 3], node: &mut Node) -> Option<Node> {
    let flattened: Vec<&Option<PlayerType>> = board.iter().flatten().collect();
    let match_child = node.children.iter().enumerate().find(|(idx, n)| {
        n.static_node_state.iter().flatten().all(|v| match v {
            Some(v) => match flattened[*idx] {
                Some(fv) => v == fv,
                None => false,
            },
            None => false,
        })
    });
    Some(match_child?.1.clone())
}

pub fn make_move(node: &mut Node) -> BoardFieldVal {
    compute_optimal(node);
    let mut val = BoardFieldVal { col: 1, row: 1 };
    if let Some(n) = node.optimal_child.take() {
        let next_flat = n.static_node_state;
        for (idx, node_val) in node.static_node_state.iter().enumerate() {
            for (inn_idx, inn_node) in node_val.iter().enumerate() {
                if let Some(v) = inn_node {
                    if let Some(nv) = &next_flat[idx][inn_idx] {
                        if nv == v {
                            val.col = inn_idx as u32;
                            val.row = idx as u32;
                        }
                    }
                }
            }
        }
    };
    val
}
