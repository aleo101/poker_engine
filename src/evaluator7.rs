use crate::dptables::SUITS;
use crate::hash::hash_quinary;
use crate::hash_table7::NOFLUSH7;
use crate::hashtable::FLUSH;

pub static BINARIES_BY_ID: [i32; 52] = [
    0x1, 0x1, 0x1, 0x1, 0x2, 0x2, 0x2, 0x2, 0x4, 0x4, 0x4, 0x4, 0x8, 0x8, 0x8, 0x8, 0x10, 0x10,
    0x10, 0x10, 0x20, 0x20, 0x20, 0x20, 0x40, 0x40, 0x40, 0x40, 0x80, 0x80, 0x80, 0x80, 0x100,
    0x100, 0x100, 0x100, 0x200, 0x200, 0x200, 0x200, 0x400, 0x400, 0x400, 0x400, 0x800, 0x800,
    0x800, 0x800, 0x1000, 0x1000, 0x1000, 0x1000,
];

pub static SUITBIT_BY_ID: [i32; 52] = [
    0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200,
    0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200,
    0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200, 0x1, 0x8, 0x40, 0x200,
    0x1, 0x8, 0x40, 0x200,
];

pub fn evaluate_7cards(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32) -> i32 {
    let mut suit_hash: i32 = 0;
    suit_hash += SUITBIT_BY_ID[a as usize];
    suit_hash += SUITBIT_BY_ID[b as usize];
    suit_hash += SUITBIT_BY_ID[c as usize];
    suit_hash += SUITBIT_BY_ID[d as usize];
    suit_hash += SUITBIT_BY_ID[e as usize];
    suit_hash += SUITBIT_BY_ID[f as usize];
    suit_hash += SUITBIT_BY_ID[g as usize];

    let mut suit_binary: [i32; 4] = [0; 4];

    if SUITS[suit_hash as usize] != 0 {
        suit_binary[(a & 0x3) as usize] |= BINARIES_BY_ID[a as usize];
        suit_binary[(b & 0x3) as usize] |= BINARIES_BY_ID[b as usize];
        suit_binary[(c & 0x3) as usize] |= BINARIES_BY_ID[c as usize];
        suit_binary[(d & 0x3) as usize] |= BINARIES_BY_ID[d as usize];
        suit_binary[(e & 0x3) as usize] |= BINARIES_BY_ID[e as usize];
        suit_binary[(f & 0x3) as usize] |= BINARIES_BY_ID[f as usize];
        suit_binary[(g & 0x3) as usize] |= BINARIES_BY_ID[g as usize];

        return FLUSH[suit_binary[(SUITS[suit_hash as usize] - 1) as usize] as usize];
    }

    let mut quinary: [u8; 13] = [0; 13];

    quinary[(a >> 2) as usize] += 1;
    quinary[(b >> 2) as usize] += 1;
    quinary[(c >> 2) as usize] += 1;
    quinary[(d >> 2) as usize] += 1;
    quinary[(e >> 2) as usize] += 1;
    quinary[(f >> 2) as usize] += 1;
    quinary[(g >> 2) as usize] += 1;

    let hash: u32 = hash_quinary(&quinary, 7);

    return NOFLUSH7[hash as usize];
}
