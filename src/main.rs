/*
 *  Copyright 2021 Alexander Leones
 *  Copyright 2016 Henry Lee
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *
 *  Copied, adapted, and modified from [PH Evaluator](https://github.com/HenryRLee/PokerHandEvaluator) to
 *  convert the 7 card evaluation alogorithm along with its supporting code to Rust code.
 */

/*
 * Written by: Alexander Leones
 * This is a seven card poker hand evaluator written in Rust I wrote to practice and learn the Rust Programming Language.
 * This is a close modification of the Algorithm invented by Henry R Lee. 
 * ...You can find Henry's original Evaluator C++ code in the license disclosure above.
 */

// mod card;
// mod deck;
pub mod dptables;
pub mod evaluator;
pub mod evaluator7;
//mod hand_ranks;
pub mod hash;
pub mod hash_table7;
pub mod hashtable;
pub mod new_card;
pub mod new_hand;
pub mod rank;
pub mod seven_four_six_two;
use crate::new_card::Card;
// use crate::card::Card;
// use crate::card::Suit;
// use crate::card::Value;
// use crate::deck::Deck;
// use crate::deck::Hand;

fn main() {
    // let card = Card::from_names("C", "2");

    // let v1 = Value::Jack;
    // let v2 = Value::Ten;

    // println!("{:?}", v1 == v2);
    // println!("{:?}", card);

    // let mut deck = Deck::create();
    // println!("Length of deck: {}", deck.get_len());
    // println!("{:?}", deck.card_deck.get(9).unwrap().get_suit());
    // println!("{:?}", deck.card_deck.get(9).unwrap().get_value());
    // let hand_one = Hand::create(deck.deal(2));
    // println!("Hand One cards: {:?}", hand_one);
    // let mut hand_two = Hand::create_empty();
    // hand_two.add_card(*deck.deal(1).get(0).unwrap());
    // println!("Hand Two cards: {:?}", hand_two);
    // println!("Length of deck: {}", deck.get_len());

    // // let three_of_kind_vec = vec![
    // //     Card::from_vals(Suit::Clubs, Value::Ace),
    // //     Card::from_vals(Suit::Diamonds, Value::Ace),
    // //     Card::from_vals(Suit::Hearts, Value::Ace),
    // //     //Card::from_vals(Suit::Spades, Value::Ace),
    // // ];

    // let royal_flush_vec = vec![
    //     Card::from_vals(Suit::Clubs, Value::Ace),
    //     Card::from_vals(Suit::Clubs, Value::Jack),
    //     Card::from_vals(Suit::Clubs, Value::Ten),
    //     Card::from_vals(Suit::Clubs, Value::Queen),
    //     Card::from_vals(Suit::Clubs, Value::King),
    // ];
    // let hand_three = Hand::create(royal_flush_vec);
    // println!(
    //     "{:?}",
    //     hand_ranks::HandRanking::calc_ranking(&hand_three.hand_cards)
    // );

    // println!("{}", hand_ranks::is_straight(&hand_three.hand_cards));
    // //println!("Suit: {} Value: {}.", card.get_suit(), card.get_value());
    let _a = new_card::Card::from_name("Qc".to_string());
    let rank1 = evaluator::evaluate_cards(
        &Card::from_name("9c".to_string()),
        &Card::from_name("4c".to_string()),
        &Card::from_name("4s".to_string()),
        &Card::from_name("9d".to_string()),
        &Card::from_name("4h".to_string()),
        &Card::from_name("Qc".to_string()),
        &Card::from_name("6c".to_string()),
    );
    let rank2 = evaluator::evaluate_cards(
        &Card::from_name("9c".to_string()),
        &Card::from_name("4c".to_string()),
        &Card::from_name("4s".to_string()),
        &Card::from_name("9d".to_string()),
        &Card::from_name("4h".to_string()),
        &Card::from_name("2c".to_string()),
        &Card::from_name("9c".to_string()),
    );

    let rank3 = evaluator::evaluate_cards(
        &Card::from_name("Tc".to_string()),
        &Card::from_name("Jc".to_string()),
        &Card::from_name("Qc".to_string()),
        &Card::from_name("Kc".to_string()),
        &Card::from_name("Ac".to_string()),
        &Card::from_name("2c".to_string()),
        &Card::from_name("9c".to_string()),
    );
    println!("{}", rank2 > rank1);
    println!("{}", rank2 < rank1);
    println!("{}", rank1.value());
    println!("{}", rank2.value());
    println!("{}", rank3.value());
}
