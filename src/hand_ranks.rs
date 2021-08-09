use crate::card::Card;
use crate::card::Value;
// use crate::deck::Deck;
// use crate::deck::Hand;
use Iterator;
#[derive(Debug)]
pub enum HandRanking {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl HandRanking {
    //TODO: Change to taking Hand in as parameter .
    pub fn calc_ranking(cards: &Vec<Card>) -> HandRanking {
        if is_flush(&cards) {
            if is_straight(&cards) {
                if is_royal_flush(&cards) {
                    return HandRanking::RoyalFlush;
                }
                return HandRanking::StraightFlush;
            }
            HandRanking::Flush
        } else if is_straight(&cards) {
            return HandRanking::Straight;
        } else {
            // cnt keeps track of the maximum number of similarly ranked cards.
            let mut cnt: usize = 0;
            let mut last_cnt: usize = 0;
            for c in cards.iter() {
                let cnt_loop = cards
                    .iter()
                    .filter(|&n| *n.get_value() == *c.get_value())
                    .count();
                if cnt_loop > cnt {
                    last_cnt = cnt;
                    cnt = cnt_loop;
                }
            }
            if (cnt == 2) && (last_cnt == 2) {
                return HandRanking::TwoPair;
            }
            match cnt {
                2 => HandRanking::Pair,
                3 => HandRanking::ThreeOfAKind,
                4 => HandRanking::FourOfAKind,
                //TODO: change this to error handle instead of incorrectly returing RF.
                _ => HandRanking::RoyalFlush,
            }
        }
    }
}

fn is_flush(cards_vector: &Vec<Card>) -> bool {
    let mut cards_iter = cards_vector.iter();
    let suit_to_match = cards_iter.next().unwrap().get_suit();
    for c in cards_iter {
        if c.get_suit() != suit_to_match {
            return false;
        }
    }
    true
}

fn is_royal_flush(cards_vector: &Vec<Card>) -> bool {
    let mut royal_cards = vec![
        Value::Ace,
        Value::King,
        Value::Queen,
        Value::Jack,
        Value::Ten,
    ];
    let cards_iter = cards_vector.iter();
    for c in cards_iter {
        if royal_cards.contains(c.get_value()) {
            let index = royal_cards.iter().position(|x| *x == *c.get_value());
            royal_cards.remove(index.unwrap());
        } else {
            return false;
        }
    }
    return true;
}

pub fn is_straight(cards_vector: &Vec<Card>) -> bool {
    let mut cards_vals: Vec<Value> = vec![];
    for c in cards_vector.iter() {
        cards_vals.push(*c.get_value());
    }
    println!("Unsorted hand: {:?}", cards_vals);
    cards_vals.sort();
    println!("Sorted hand: {:?}", cards_vals);
    let mut cards_vals_iter = cards_vals.iter();
    let mut last_point_value: i32 = cards_vals_iter.next().unwrap().get_points();
    if last_point_value == 14 {
        last_point_value = 1;
    }
    for c in cards_vals_iter {
        if (c.get_points() - 1) != last_point_value {
            return false;
        }
        last_point_value = c.get_points();
    }
    true
}
// fn score(hand: Hand) {
//     let ranks = "23456789TJQKA";
//     if

// }
