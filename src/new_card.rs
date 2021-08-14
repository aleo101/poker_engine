//card.h equivalent.
use phf::phf_map;

const RANK_MAP: phf::Map<char, i32> = phf_map! {
     '2' => 0 ,  '3' => 1 ,  '4'=> 2 , '5' => 3 ,
     '6' => 4 ,  '7' => 5 ,  '8'=> 6 , '9' => 7 ,
     'T' => 8 ,  'J' => 9 ,  'Q'=> 10 , 'K' => 11 ,  'A' => 12 ,
};

const SUIT_MAP: phf::Map<char, i32> = phf_map! {
      'C'  => 0    ,   'D'  => 1    ,  'H'  => 2   , 'S'  => 3,
      'c'  => 0    ,   'd'  => 1    ,  'h'  => 2   , 's'  => 3,
};

pub struct Card {
    id_: i32,
}

impl Card {
    pub fn from_id(id_: i32) -> Card {
        Card { id_ }
    }

    pub fn from_name(name: String) -> Card {
        if name.len() < 2 {
            todo!("Throw an exception here");
        }
        let result = match RANK_MAP.get(&name.chars().nth(0).unwrap()) {
            Some(a) => a,
            None => &499, // Just using this as an error code.
        };

        let result2 = match SUIT_MAP.get(&name.chars().nth(1).unwrap()) {
            Some(a) => a,
            None => &500,
        };

        Card {
            id_: result * 4 + result2,
        }
    }
    pub fn int(&self) -> usize {
        self.id_ as usize
    }
}
