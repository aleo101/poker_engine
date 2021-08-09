use std::slice::Iter;
use std::{cmp::Ordering, str::FromStr};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug)]
pub struct SuitFromStrError(String);

impl FromStr for Suit {
    type Err = SuitFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Suit::Spades),
            "H" => Ok(Suit::Hearts),
            "D" => Ok(Suit::Diamonds),
            "C" => Ok(Suit::Clubs),
            _ => Err(SuitFromStrError(s.to_string())),
        }
    }
}

impl Suit {
    pub fn iterator() -> Iter<'static, Suit> {
        static SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
        SUITS.iter()
    }
}

#[derive(Copy, Clone, Eq, Debug)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Value {
    pub fn get_points(&self) -> i32 {
        match self {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 11,
            Value::Queen => 12,
            Value::King => 13,
            Value::Ace => 14,
        }
    }

    pub fn iterator() -> Iter<'static, Value> {
        static VALUES: [Value; 13] = [
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
            Value::Ace,
        ];

        VALUES.iter()
    }
}

#[derive(Debug)]
pub struct ValueFromStrError(String);
impl FromStr for Value {
    type Err = ValueFromStrError;
    fn from_str(string_in: &str) -> Result<Self, Self::Err> {
        match string_in {
            "2" => Ok(Value::Two),
            "3" => Ok(Value::Three),
            "4" => Ok(Value::Four),
            "5" => Ok(Value::Five),
            "6" => Ok(Value::Six),
            "7" => Ok(Value::Seven),
            "8" => Ok(Value::Eight),
            "9" => Ok(Value::Nine),
            "10" => Ok(Value::Ten),
            "J" => Ok(Value::Jack),
            "Q" => Ok(Value::Queen),
            "K" => Ok(Value::King),
            "A" => Ok(Value::Ace),
            _ => Err(ValueFromStrError("Illegal Value".to_string())),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Value) -> Ordering {
        self.get_points().cmp(&other.get_points())
    }
}

// impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        self.get_points() == other.get_points()
    }
}

//     fn ne(&self, other: &Self) -> bool {
//         !self.get_points().eq(other.get_points())
//     }
// }

#[derive(Copy, Clone, Debug)]
pub struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    pub fn get_suit(&self) -> &Suit {
        &self.suit
    }
    pub fn get_value(&self) -> &Value {
        &self.value
    }
    pub fn from(suit: &str, value: &str) -> Card {
        Card {
            suit: match suit.parse::<Suit>() {
                Ok(s) => s,
                Err(e) => panic!("Not a valid suit: {:?}", e),
            },
            value: value.parse().unwrap(),
        }
    }
    pub fn from_vals(suit: Suit, value: Value) -> Card {
        Card { suit, value }
    }
}
