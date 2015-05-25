//! holdem-rs contains more or less common types and functions used by other poker related libraries.

extern crate cards;

use std::cmp::{Ord};

use cards::card::{Card};

/// All hand rank classes that a 5-card hand can be worth in Texas Hold'em.
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

/// A card encoded using the bit pattern described in Cactus Kev's
/// [article](http://www.suffecool.net/poker/evaluator.html).
pub type CactusKevCard = u32;

/// A value representing the strength of a hand. The higheer, the better.
/// The numbers go from 0 to 7461 inclusive.
pub type HandRank = u16;
pub const HAND_RANK_COUNT : u16 = 7462;

/// Translates a hand rank to a rank class.
/// assumes there are HAND_RANK_COUNT distinct hand ranks, where the
/// largest are the most valuable. Numbers based on: http://www.suffecool.net/poker/evaluator.html
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

// TODO: this could be an option just as well.
/// When a card can be either delt or pending.
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
