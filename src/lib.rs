extern crate cards;

use std::cmp::{Ord};

use cards::card::{Card};

#[derive(Debug, PartialEq)]
#[derive(Eq, Hash)]
#[derive(PartialOrd, Ord)]
pub enum HandRankClass {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

// find maximum value to determine best hand
// larger values are "better", inverting the Cactus Kev convention
pub type HandRank = u16;

pub const HAND_RANK_COUNT : u16 = 7462;

// assumes there are HAND_RANK_COUNT distinct hand ranks, where the
// largest are the most valuable. Instead of leaving the largest ones as the worst (7462 to 1 inclusive)
// let's invert this standard to be (0 to 7461 inclusive), where the largest is best.
// numbers based on: http://www.suffecool.net/poker/evaluator.html
pub fn hand_rank_to_class(val: &HandRank) -> HandRankClass {
    if *val < 1277 {
        HandRankClass::HighCard
    } else if *val < 1277+2860 {
        HandRankClass::OnePair
    } else if *val < 1277+2860+858 {
        HandRankClass::TwoPair
    } else if *val < 1277+2860+858+858 {
        HandRankClass::ThreeOfAKind
    } else if *val < 1277+2860+858+858+10 {
        HandRankClass::Straight
    } else if *val < 1277+2860+858+858+10+1277 {
        HandRankClass::Flush
    } else if *val < 1277+2860+858+858+10+1277+156 {
        HandRankClass::FullHouse
    } else if *val < 1277+2860+858+858+10+1277+156+156 {
        HandRankClass::FourOfAKind
    } else if *val < 1277+2860+858+858+10+1277+156+156+10 {
        HandRankClass::StraightFlush
    } else {
        panic!("Unexpected hand rank value! '{}'", *val)
    }
}

pub enum CardSlot {
    Empty,
    Dealt(Card),
}

impl CardSlot {
    pub fn expect_borrow(&self) -> &Card {
        match self {
            &CardSlot::Empty => panic!("You are trying to borrow a non-dealt card"),
            &CardSlot::Dealt(ref x) => x
        }
    }
}

pub struct HandCards(pub CardSlot, pub CardSlot);
pub struct CommunityCards(pub CardSlot, pub CardSlot, pub CardSlot, pub CardSlot, pub CardSlot);
