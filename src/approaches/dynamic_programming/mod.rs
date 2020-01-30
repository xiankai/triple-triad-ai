pub mod tree;
#[allow(unused_imports)]
use rand::Rng;
use tree::Tree;

use crate::data_structures::{rules::Rule, Action, Card, State};

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
    for decision in compute_decisions(&state, &mut 0) {
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

fn compute_decisions(state: &State, counter: &mut u32) -> Vec<Tree> {
    let actions = compute_actions(state);

    let mut decisions = vec![];
    for action in actions.into_iter() {
        *counter += 1;
        let next_state = compute_next_state(state, &action);
        let children = compute_decisions(&next_state, counter);
        let amortized_value = evaluate(&next_state, &children);
        let decision = Tree {
            state: next_state,
            action: Some(action),
            amortized_value,
            children,
        };
        decisions.push(decision);
    }
    decisions
}

fn compute_next_state(state: &State, action: &Action) -> State {
    let mut next_state = *state;
    // flip flip flip!
    next_state.take_action(action);
    // then do opponent's action!
    play_for_opponent(next_state)
}

fn play_for_opponent(mut state: State) -> State {
    state.invert();
    let actions = compute_actions(&state);
    if actions.len() <= 0 {
        state.invert();
        return state;
    }
    // sample a random action
    let mut rng = rand::thread_rng();
    let random_action = &actions[rng.gen_range(0, actions.len())];
    state.take_action(random_action);

    // take first action
    // state.take_action(&actions[0]);

    state.invert();
    state
}

fn compute_actions(state: &State) -> Vec<Action> {
    let mut actions = vec![];
    for (hand_position, card) in state.hand.into_iter().enumerate() {
        if card.is_some() {
            for (board_position, position) in state.board.iter().enumerate() {
                if position.is_none() {
                    actions.push(Action {
                        card_in_hand: hand_position,
                        position: board_position,
                    })
                }
            }
        }
    }
    actions
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
