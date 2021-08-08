pub mod tree;
use tree::Tree;
pub mod check_all_opponent_actions;

use crate::data_structures::{State};

pub fn dp(state: State, mut decision_counter: &mut u32) {
    let mut amortized_value_board = [0.0 as f32; 9];
    for decision in check_all_opponent_actions::compute_decisions(&state, &mut decision_counter) {
        if let Some(action) = decision.action {
            amortized_value_board[action.position] = decision.amortized_value;
        }
    }

    let mut counter = 0;
    for val in amortized_value_board.iter() {
        print!("{: ^15}", val);
        counter += 1;
        if counter % 3 == 0 {
            println!();
        }
    }
}

fn evaluate(state: &State, decisions: &Vec<Tree>) -> f32 {
    if decisions.len() <= 0 {
        return check_score(state);
    };
    let mut score = 0.0;
    for decision in decisions.iter() {
        score += decision.amortized_value;
    }
    score / decisions.len() as f32
}

fn check_score(state: &State) -> f32 {
    let mut score = 0;
    for position in state.board.iter() {
        if let Some(position) = position {
            if position.is_player {
                score += 1
            } else {
                score -= 1
            }
        }
    }

    // check if we have a card left.
    // if not, means we have used all cards, and the winning threshold is higher.
    if state.hand.iter().find(|c| c.is_some()).is_some() {
    } else {
        score -= 1;
    }

    if score > 0 {
        1.0
    } else if score < 0 {
        -1.0
    } else {
        0.0
    }
}
