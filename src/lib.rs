extern crate cards;

use cards::card::{Card};

//TODO: this could be an option as well
//pub type CardSlot = Option<Card>;

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

#[derive(Debug)]
#[derive(PartialEq)] 
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


pub struct HandCards(pub CardSlot, pub CardSlot);
pub struct CommunityCards(pub CardSlot, pub CardSlot, pub CardSlot, pub CardSlot, pub CardSlot);
