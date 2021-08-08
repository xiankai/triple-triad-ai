use super::evaluate;
use super::tree::Tree;
use crate::data_structures::{Action, State};
use rand::Rng;

pub fn compute_decisions(state: &State, counter: &mut u32) -> Vec<Tree> {
    let mut decisions = vec![];
    for action in compute_actions(state).into_iter() {
        *counter += 1;
        if *counter % 100000 == 0 {
            println!("{}", counter);
        }
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
