use std::option::Option;
pub mod rules;
use rules::Rule;

pub type Strength = i8; // -8-18 (ascension/descension)

pub type Star = u8; // 1-5

pub type HandPosition = usize; // 0-4

pub type BoardPosition = usize; // 0-8

#[allow(dead_code)] // Work-around rust-lang/rust#64362
#[derive(Copy, Clone, Debug)]
pub enum CardType {
    BEASTMAN = 0,
    GARLAND = 1,
    PRIMAL = 2,
    SCIONS = 3,
}

#[derive(Copy, Clone, Debug)]
pub struct Card {
    // pub name: String,
    pub top: Strength,
    pub bottom: Strength,
    pub left: Strength,
    pub right: Strength,
    pub star: Star,
    pub card_type: Option<CardType>,
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub card: Card,
    pub is_player: bool,
}

pub type Board = [Option<Position>; 9 as BoardPosition];

pub type Hand = [Option<Card>; 5];

#[allow(dead_code)]
pub type Deck = Vec<Card>;

pub type Rules = [Option<Rule>; 4];

#[derive(Copy, Clone, Debug)]
pub struct State {
    pub board: Board,
    pub hand: Hand,
    pub opponent: Hand,
    pub rules: Rules,
}

/**
 * 0  1  2
 * 3  4  5
 * 6  7  8
 */
fn find_adjacent_positions(pos: BoardPosition) -> Vec<BoardPosition> {
    let mut positions = vec![];
    if pos > 2 {
        // has top side
        positions.push(pos - 3) // 5
    }
    if pos % 3 != 0 {
        // has left side
        positions.push(pos - 1)
    }
    if pos < 6 {
        // has bottom side
        positions.push(pos + 3)
    }
    if pos % 3 != 2 {
        // has right side
        positions.push(pos + 1)
    }
    positions
}

fn should_flip(placed_card: Strength, opposing_card: Strength, rules: &Rules) -> bool {
    if rules.contains(&Some(Rule::FallenAce))
        && placed_card == 1 as Strength
        && opposing_card == 10 as Strength
    {
        return true;
    }

    if rules.contains(&Some(Rule::Reverse)) && placed_card < opposing_card {
        return true;
    }

    if placed_card > opposing_card {
        return true;
    }
    false
}

impl State {
    pub fn take_action(&mut self, action: &Action) {
        if let Some(card) = self.hand[action.card_in_hand] {
            assert!(self.board[action.position].is_none());
            self.board[action.position] = Some(Position {
                card,
                is_player: true,
            });
            self.hand[action.card_in_hand] = None;
        } else {
            panic!("not supposed to happen :(");
        }

        let flipped_positions = self.flip_positions(action.position, true);

        if self.rules.contains(&Some(Rule::Plus)) && self.check_plus(action.position) {
            self.do_combo(action.position);
        }

        if self.rules.contains(&Some(Rule::Same)) && self.check_same(action.position) {
            self.do_combo(action.position);
        }
    }

    fn compare_positions(
        &self,
        board_position: BoardPosition,
        opposite_board_position: BoardPosition,
    ) -> (Strength, Strength) {
        if let Some(position) = &self.board[board_position] {
            if let Some(opposite_position) = &self.board[opposite_board_position] {
                let mut card_modifier = 0;
                let mut opposite_card_modifier = 0;

                if self.rules.contains(&Some(Rule::Ascension))
                    || self.rules.contains(&Some(Rule::Descension))
                {
                    let modifiers = self.check_type_modifiers();
                    if let Some(card_type) = &position.card.card_type {
                        card_modifier += modifiers[*card_type as usize];
                    }
                    if let Some(card_type) = &opposite_position.card.card_type {
                        opposite_card_modifier += modifiers[*card_type as usize];
                    }
                    if self.rules.contains(&Some(Rule::Descension)) {
                        card_modifier = -card_modifier;
                        opposite_card_modifier = -opposite_card_modifier;
                    }
                }

                // +3 to avoid overflow for an usize
                match board_position + 3 - opposite_board_position {
                    6 => {
                        return (
                            position.card.top + card_modifier,
                            opposite_position.card.bottom + opposite_card_modifier,
                        )
                    }
                    0 => {
                        return (
                            position.card.bottom + card_modifier,
                            opposite_position.card.top + opposite_card_modifier,
                        )
                    }
                    4 => {
                        return (
                            position.card.left + card_modifier,
                            opposite_position.card.right + opposite_card_modifier,
                        )
                    }
                    2 => {
                        return (
                            position.card.right + card_modifier,
                            opposite_position.card.left + opposite_card_modifier,
                        )
                    }
                    _ => panic!(
                        "This is not supposed to happen. Compared {:?} and {:?} positions",
                        position, opposite_position
                    ),
                }
            }
        }
        panic!(
            "This is not supposed to happen. Compared {} and {} positions",
            board_position, opposite_board_position
        );
    }

    pub fn flip_positions(&mut self, position: BoardPosition, check: bool) -> Vec<BoardPosition> {
        let mut positions_flipped = vec![];
        for adjacent_position in find_adjacent_positions(position).into_iter() {
            if let Some(current_position) = self.board[adjacent_position] {
                if !current_position.is_player {
                    let (strength, opposite_strength) =
                        self.compare_positions(position, adjacent_position);
                    if current_position.is_player == false
                        && (check == false || should_flip(strength, opposite_strength, &self.rules))
                    {
                        self.board[adjacent_position] = Some(Position {
                            card: current_position.card,
                            is_player: true,
                        });
                        positions_flipped.push(adjacent_position);
                    }
                }
            }
        }
        positions_flipped
    }

    fn check_type_modifiers(&self) -> [Strength; 4] {
        let mut types = [0 as Strength; 4];
        for position in self.board.iter() {
            if let Some(position) = &position {
                match position.card.card_type {
                    Some(CardType::BEASTMAN) => types[CardType::BEASTMAN as usize] += 1,
                    Some(CardType::GARLAND) => types[CardType::GARLAND as usize] += 1,
                    Some(CardType::PRIMAL) => types[CardType::PRIMAL as usize] += 1,
                    Some(CardType::SCIONS) => types[CardType::SCIONS as usize] += 1,
                    _ => {}
                }
            }
        }
        types
    }

    fn check_plus(&self, position: BoardPosition) -> bool {
        let mut sums = vec![];
        for adjacent_position in find_adjacent_positions(position).into_iter() {
            if let Some(_) = self.board[adjacent_position] {
                let (strength, opposite_strength) =
                    self.compare_positions(position, adjacent_position);
                if self.rules.contains(&Some(Rule::Plus)) {
                    let sum = strength + opposite_strength;
                    if sums.contains(&sum) {
                        return true;
                    } else {
                        sums.push(sum);
                    }
                }
            }
        }
        false
    }

    fn check_same(&self, position: BoardPosition) -> bool {
        for adjacent_position in find_adjacent_positions(position).into_iter() {
            if let Some(_) = self.board[adjacent_position] {
                let (strength, opposite_strength) =
                    self.compare_positions(position, adjacent_position);
                if self.rules.contains(&Some(Rule::Same)) {
                    if strength == opposite_strength {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn do_combo(&mut self, board_position: BoardPosition) {
        let mut flipped_positions: Vec<BoardPosition> = vec![];
        for adjacent_position in find_adjacent_positions(board_position).into_iter() {
            // Sketchy assignment
            if let Some(mut position) = self.board[adjacent_position] {
                if position.is_player == false {
                    position.is_player = true;
                    flipped_positions.push(adjacent_position);
                }
            }
        }
        while flipped_positions.len() > 0 {
            if let Some(flipped_position) = flipped_positions.pop() {
                flipped_positions.extend(self.flip_positions(flipped_position, false));
            }
        }
    }

    pub fn invert(&mut self) {
        let temp = self.hand;
        self.hand = self.opponent;
        self.opponent = temp;
        for (board_position, _) in self.board.clone().into_iter().enumerate() {
            if let Some(mut position) = self.board[board_position] {
                position.is_player = !position.is_player;
                self.board[board_position] = Some(position);
            }
        }
    }

    #[allow(dead_code)]
    pub fn debug(&self, string: &str) {
        let is_players: Vec<isize> = self
            .board
            .into_iter()
            .map(|x| {
                if let Some(x) = x {
                    if x.is_player {
                        return 1;
                    } else {
                        return 0;
                    }
                }
                -1
            })
            .collect();
        println!("{}: result board: {:?}", string, is_players);
    }
}

#[derive(Debug)]
pub struct Action {
    pub position: BoardPosition,
    pub card_in_hand: HandPosition,
}
