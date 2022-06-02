use rand::prelude::*;
use std::fmt::*;
use std::result::Result;

pub struct Card {
    rank: char,
    suit: char,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        if self.rank == 'T' {
            write!(f, "10{}", self.suit)
        } else {
            write!(f, "{}{}", self.rank, self.suit)
        }
    }
}

const RANKS: &str = "23456789TJQKA";
const SUITS: &str = "CDHS";

pub fn new_deck() -> Vec<Card> {
    let mut rng = thread_rng();
    let mut deck = Vec::new();

    for rank in RANKS.chars() {
        for suit in SUITS.chars() {
            deck.push(Card {
                rank: rank,
                suit: suit,
            });
        }
    }

    deck.shuffle(&mut rng);

    deck
}
