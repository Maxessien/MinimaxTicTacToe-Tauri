
#[derive(Clone, Copy)]
pub enum BoardValues {
    High = 1, Draw = 0, Low = -1, Null = 4
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlayerType { Maximizer, Minimizer }

#[derive(Clone)]
pub struct Node {
  pub level_player: PlayerType,
  pub children: Vec<Node>,
  pub is_leaf: bool,
  pub is_root: bool,
  pub optimal_child: Option<Box<Node>>,
  pub score: BoardValues,
  pub static_node_state: [[Option<PlayerType>; 3]; 3],
  pub depth: u32,
}

#[derive(Clone)]
pub struct Optimal {pub optimal: Node, pub depth: u32}

pub struct BoardFieldVal {
    pub row: u32, pub col: u32
}