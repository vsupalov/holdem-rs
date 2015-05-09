extern crate cards;

use cards::card::{Card};

pub enum CardSlot {
    Empty,
    Card,
}

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


pub struct HandCards(pub CardSlot, pub CardSlot);
pub struct CommunityCards(pub CardSlot, pub CardSlot, pub CardSlot, pub CardSlot, pub CardSlot);
//TODO: unwrap to go from cardSlot to card or panic?
