pub mod tree;
use tree::Tree;
pub mod opponent_plays_random_action;

use crate::data_structures::{rules::Rule, Card, State};

pub fn dp() {
    let card = Card {
        top: 3,
        bottom: 3,
        left: 6,
        right: 3,
        star: 1,
        card_type: None,
    };

    let state = State {
        board: [None; 9],
        hand: [Some(card); 5],
        opponent: [Some(card); 5],
        rules: [None, None, None, Some(Rule::Plus)],
    };

    let mut amortized_value_board = [0.0 as f32; 9];
    for decision in opponent_plays_random_action::compute_decisions(&state, &mut 0) {
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
