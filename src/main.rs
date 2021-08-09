mod card;
mod deck;
mod hand_ranks;
use crate::card::Card;
use crate::card::Suit;
use crate::card::Value;
use crate::deck::Deck;
use crate::deck::Hand;

fn main() {
    let card = Card::from("C", "2");

    let v1 = Value::Jack;
    let v2 = Value::Ten;

    println!("{:?}", v1 == v2);
    println!("{:?}", card);

    let mut deck = Deck::create();
    println!("Length of deck: {}", deck.get_len());
    println!("{:?}", deck.card_deck.get(9).unwrap().get_suit());
    println!("{:?}", deck.card_deck.get(9).unwrap().get_value());
    let hand_one = Hand::create(deck.deal(2));
    println!("Hand One cards: {:?}", hand_one);
    let mut hand_two = Hand::create_empty();
    hand_two.add_card(*deck.deal(1).get(0).unwrap());
    println!("Hand Two cards: {:?}", hand_two);
    println!("Length of deck: {}", deck.get_len());

    // let three_of_kind_vec = vec![
    //     Card::from_vals(Suit::Clubs, Value::Ace),
    //     Card::from_vals(Suit::Diamonds, Value::Ace),
    //     Card::from_vals(Suit::Hearts, Value::Ace),
    //     //Card::from_vals(Suit::Spades, Value::Ace),
    // ];

    let royal_flush_vec = vec![
        Card::from_vals(Suit::Clubs, Value::Ace),
        Card::from_vals(Suit::Clubs, Value::Jack),
        Card::from_vals(Suit::Clubs, Value::Ten),
        Card::from_vals(Suit::Clubs, Value::Queen),
        Card::from_vals(Suit::Clubs, Value::King),
    ];
    let hand_three = Hand::create(royal_flush_vec);
    println!(
        "{:?}",
        hand_ranks::HandRanking::calc_ranking(&hand_three.hand_cards)
    );

    println!("{}", hand_ranks::is_straight(&hand_three.hand_cards));
    //println!("Suit: {} Value: {}.", card.get_suit(), card.get_value());
}
