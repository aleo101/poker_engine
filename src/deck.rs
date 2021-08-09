use crate::card::Card;
use crate::card::{Suit, Value};
pub struct Deck {
    pub card_deck: Vec<Card>,
}

impl Deck {
    pub fn create() -> Self {
        let mut cards = vec![];
        for s in Suit::iterator() {
            for v in Value::iterator() {
                cards.push(Card::from_vals(*s, *v));
            }
        }
        Deck { card_deck: cards }
    }

    pub fn deal(&mut self, cnt: i32) -> Vec<Card> {
        let mut returned_cards = vec![];
        for _ in 0..cnt {
            returned_cards.push(self.card_deck.pop().expect("No more cards to deal..."));
        }
        returned_cards
    }
    //TODO: Should I use a trait?
    pub fn get_len(&self) -> usize {
        self.card_deck.len()
    }
}

//TODO: consider mkaing the fuctionality of deck and hand traits.
#[derive(Debug)]
pub struct Hand {
    pub hand_cards: Vec<Card>,
}

impl Hand {
    pub fn create(cards: Vec<Card>) -> Self {
        Hand { hand_cards: cards }
    }

    pub fn create_empty() -> Self {
        Hand { hand_cards: vec![] }
    }

    pub fn add_card(&mut self, card: Card) {
        self.hand_cards.push(card);
    }
}

// impl Debug for Hand {
//     fn fmt(&self, )
// }
