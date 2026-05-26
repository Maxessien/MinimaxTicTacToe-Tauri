use serde::{Serialize, Deserialize};
use wincode::{SchemaRead, SchemaWrite};


#[derive(Clone, Copy, Deserialize, SchemaWrite, SchemaRead, Debug, PartialEq)]
#[repr(C)] 
pub enum PlayerType { Maximizer, Minimizer }

#[derive(Deserialize, Clone, SchemaWrite, SchemaRead, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[repr(C)] 
pub struct Node {
  pub level_player: PlayerType,
  pub children: Vec<Node>,
  pub is_leaf: bool,
  pub is_root: bool,
  pub optimal_child: Option<usize>,
  pub score: Option<i32>,
  pub static_node_state: [[Option<PlayerType>; 3]; 3],
  pub depth: u32,
}

#[derive(Clone)]
pub struct Optimal<'a> {pub optimal: &'a Node, pub depth: u32}

#[derive(Serialize)]
pub struct BoardFieldVal {
    pub row: u32, pub col: u32
}