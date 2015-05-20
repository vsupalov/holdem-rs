extern crate cards;

use cards::card::{Card};

// TODO: impl ORDering?
#[derive(Debug, PartialEq)]
#[derive(Eq, Hash)]
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

pub type HandRank = u32; // smaller values are "better", Cactus Kev convention

//TODO: functions to compare hand ranks -> find best?

pub fn hand_rank_to_class(val: &HandRank) -> HandRankClass {
    match *val {
        x if x > 6185 => HandRankClass::HighCard,
        x if x > 3325 => HandRankClass::OnePair,
        x if x > 2467 => HandRankClass::TwoPair,
        x if x > 1609 => HandRankClass::ThreeOfAKind,
        x if x > 1599 => HandRankClass::Straight,
        x if x > 322  => HandRankClass::Flush,
        x if x > 166  => HandRankClass::FullHouse,
        x if x > 10   => HandRankClass::FourOfAKind,
        x if x > 0    => HandRankClass::StraightFlush,
        _             => panic!("Unexpected hand rank value!")
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
