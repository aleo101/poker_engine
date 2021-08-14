use crate::dptables::SUITS;
use crate::evaluator7::evaluate_7cards;
use crate::hash::hash_quinary;
use crate::hash_table7::NOFLUSH7;
use crate::hashtable::FLUSH;
use crate::new_card::Card;
use crate::new_hand::Hand;
use crate::rank::Rank;
pub fn evaluate_cards(
    a: &Card,
    b: &Card,
    c: &Card,
    d: &Card,
    e: &Card,
    f: &Card,
    g: &Card,
) -> Rank {
    return Rank::from_value(evaluate_7cards(
        a.int() as i32,
        b.int() as i32,
        c.int() as i32,
        d.int() as i32,
        e.int() as i32,
        f.int() as i32,
        g.int() as i32,
    ));
}

pub fn evaluate_hand(hand: &Hand) -> Rank {
    if SUITS[*hand.get_suit_hash() as usize] != 0 {
        return Rank::from_value(
            FLUSH[hand.get_suit_binary()[(SUITS[*hand.get_suit_hash() as usize] - 1) as usize]
                as usize],
        );
    }

    let hash: u32 = hash_quinary(hand.get_quinary(), *hand.size() as i32);

    return Rank::from_value(NOFLUSH7[hash as usize]);
}
