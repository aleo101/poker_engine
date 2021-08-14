use crate::seven_four_six_two::RANK_DESCRIPTION;
pub const RANK_CATEGORY_DESCRIPTION: [&str; 10] = [
    "",
    "Straight Flush",
    "Four of a Kind",
    "Full House",
    "Flush",
    "Straight",
    "Three of a Kind",
    "Two Pair",
    "One Pair",
    "High Card",
];

pub enum RankCategory {
    // FIVE_OF_A_KIND = 0, // Reserved
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

pub const fn get_rank_category(rank: i32) -> RankCategory {
    if rank > 6185 {
        return RankCategory::HighCard;
    } // 1277 high card
    if rank > 3325 {
        return RankCategory::OnePair;
    } // 2860 one pair
    if rank > 2467 {
        return RankCategory::TwoPair;
    } //  858 two pair
    if rank > 1609 {
        return RankCategory::ThreeOfAKind;
    } //  858 three-kind
    if rank > 1599 {
        return RankCategory::Straight;
    } //   10 straights
    if rank > 322 {
        return RankCategory::Flush;
    } // 1277 flushes
    if rank > 166 {
        return RankCategory::FullHouse;
    } //  156 full house
    if rank > 10 {
        return RankCategory::FourOfAKind;
    } //  156 four-kind
    return RankCategory::StraightFlush; //   10 straight-flushes
}

pub const fn describe_rank_category(category: RankCategory) -> &'static str {
    return RANK_CATEGORY_DESCRIPTION[category as usize];
}

pub const fn describe_rank(rank: i32) -> &'static str {
    return RANK_DESCRIPTION[rank as usize][1];
}

pub const fn describe_sample_hand(rank: i32) -> &'static str {
    return RANK_DESCRIPTION[rank as usize][0];
}

pub const fn is_flush(rank: i32) -> bool {
    let return_val: bool = match get_rank_category(rank) {
        RankCategory::StraightFlush => true,
        RankCategory::Flush => true,
        _ => false,
    };
    return_val
}

pub struct Rank {
    value_: i32,
}

impl Rank {
    pub const fn value(&self) -> i32 {
        self.value_
    }
    pub const fn category(&self) -> RankCategory {
        return get_rank_category(self.value_);
    }

    pub const fn describe_category(&self) -> &'static str {
        return describe_rank_category(self.category());
    }

    pub const fn describe_rank(&self) -> &'static str {
        return describe_rank(self.value_);
    }

    pub const fn describe_sample_hand(&self) -> &'static str {
        return describe_sample_hand(self.value_);
    }

    pub const fn is_flush(&self) -> bool {
        return is_flush(self.value_);
    }

    pub fn from_value(value: i32) -> Self {
        Self { value_: value }
    }
}

impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        return &self.value_ == &other.value_;
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.value_.partial_cmp(&self.value_)
    }
}
