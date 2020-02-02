mod approaches {
    pub mod dynamic_programming;
}

mod data_structures;
use approaches::dynamic_programming::dp;
use data_structures::{rules::Rule, Card, State};

fn main() {
    let mut decision_counter = 0 as u32;
    let starting_state = generate_starting_state();
    dp(starting_state, &mut decision_counter);
}

pub fn generate_starting_state() -> State {
    let card = Card {
        top: 3,
        bottom: 3,
        left: 6,
        right: 3,
        star: 1,
        card_type: None,
    };

    State {
        board: [None; 9],
        hand: [Some(card); 5],
        opponent: [Some(card); 5],
        rules: [None, None, None, Some(Rule::Plus)],
    }
}