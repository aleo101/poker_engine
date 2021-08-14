use crate::new_card::Card;

const BINARIES_BY_ID: [i32; 52] = [
    0x1, 0x1, 0x1, 0x1, 0x2, 0x2, 0x2, 0x2, 0x4, 0x4, 0x4, 0x4, 0x8, 0x8, 0x8, 0x8, 0x10, 0x10,
    0x10, 0x10, 0x20, 0x20, 0x20, 0x20, 0x40, 0x40, 0x40, 0x40, 0x80, 0x80, 0x80, 0x80, 0x100,
    0x100, 0x100, 0x100, 0x200, 0x200, 0x200, 0x200, 0x400, 0x400, 0x400, 0x400, 0x800, 0x800,
    0x800, 0x800, 0x1000, 0x1000, 0x1000, 0x1000,
];

const SUITBIT_BY_ID: [i32; 52] = [
    0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200,
    0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200,
    0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200,
    0x1, 0x8, 0x40, 0x200,
];

pub struct Hand {
    size_: u8,
    suit_hash: i32,
    suit_binary: [i32; 4],
    quinary_: [u8; 13],
}

impl Hand {
    pub fn from_card_vector(cards: Vec<Card>) -> Hand {
        let mut suit_hash = 0;
        let mut suit_binary: [i32; 4] = [0; 4];
        let mut quinary_: [u8; 13] = [0; 13];
        let mut size_: u8 = 0;
        for card in cards.iter() {
            suit_hash = suit_hash + SUITBIT_BY_ID[card.int()];
            suit_binary[card.int() & 0x3] |= BINARIES_BY_ID[card.int()];
            quinary_[(card.int() >> 2)] = quinary_[(card.int() >> 2)] + 1;
            size_ = size_ + 1;
        }
        Hand {
            size_,
            suit_hash,
            suit_binary,
            quinary_,
        }
    }
    pub fn from_card(card: Card) -> Hand {
        let mut suit_hash = 0;
        let mut suit_binary: [i32; 4] = [0; 4];
        let mut quinary_: [u8; 13] = [0; 13];
        let mut size_: u8 = 0;
        suit_hash = suit_hash + SUITBIT_BY_ID[card.int()];
        suit_binary[card.int() & 0x3] |= BINARIES_BY_ID[card.int()];
        quinary_[(card.int() >> 2)] = quinary_[(card.int() >> 2)] + 1;
        size_ = size_ + 1;
        Hand {
            size_,
            suit_hash,
            suit_binary,
            quinary_,
        }
    }

    pub const fn get_suit_hash(&self) -> &i32 {
        return &self.suit_hash;
    }

    pub const fn size(&self) -> &u8 {
        return &self.size_;
    }
    pub const fn get_suit_binary(&self) -> &[i32; 4] {
        return &self.suit_binary;
    }

    pub const fn get_quinary(&self) -> &[u8; 13] {
        return &self.quinary_;
    }
}
