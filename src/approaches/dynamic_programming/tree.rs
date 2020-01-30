use crate::data_structures::{Action, State};

// root node -> no action, has children
// child node -> has action, has children
// terminal node -> no action, no children

pub struct Tree {
    pub state: State,
    pub action: Option<Action>,
    pub amortized_value: f32,
    pub children: Vec<Tree>,
}
