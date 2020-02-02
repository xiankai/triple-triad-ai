use super::evaluate;
use super::tree::Tree;
use crate::data_structures::{Action, State};

pub fn compute_decisions(state: &State, counter: &mut u32) -> Vec<Tree> {
    let mut decisions = vec![];
    for action in compute_actions(state).into_iter() {
        *counter += 1;
        let next_state = compute_next_state(state, &action);
         next_state.invert();
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
    next_state.take_action(action);
    next_state
}