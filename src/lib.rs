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
// largest are the most valuable. An inversion of cactus kev's ranks.
pub fn hand_rank_to_class(val: &HandRank) -> HandRankClass {
    match *val {
        x if x < HAND_RANK_COUNT-6185 => HandRankClass::HighCard,
        x if x < HAND_RANK_COUNT-3325 => HandRankClass::OnePair,
        x if x < HAND_RANK_COUNT-2467 => HandRankClass::TwoPair,
        x if x < HAND_RANK_COUNT-1609 => HandRankClass::ThreeOfAKind,
        x if x < HAND_RANK_COUNT-1599 => HandRankClass::Straight,
        x if x < HAND_RANK_COUNT-322  => HandRankClass::Flush,
        x if x < HAND_RANK_COUNT-166  => HandRankClass::FullHouse,
        x if x < HAND_RANK_COUNT-10   => HandRankClass::FourOfAKind,
        x if x <= HAND_RANK_COUNT-0    => HandRankClass::StraightFlush, //include the former zero (now HAND_RANK_COUNT)
        x                             => panic!("Unexpected hand rank value! '{}'", x)
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
