// How does a rule work?
// -> Roulette (pick another)
// -> Win rule (Sudden Death)
// -> Deck generation (Swap, Random) -> Hand
// -> Hand display (All Open, Three Open)
// -> Hand logic (Order, Chaos)
// -> Determine if a card should flip another card? (Asc, Desc, Fallen Ace, Reverse, Same, Plus)

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Rule {
    Ascension,
    Descension,
    FallenAce,
    Plus,
    Reverse,
    Same,
    // AllOpen,
    // Chaos,
    // Order,
    // Random,
    // Swap,
    // ThreeOpen,
}
