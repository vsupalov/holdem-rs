extern crate cards;

use cards::card::{Card};

pub type CardSlot = Option<Card>;

/*
pub enum CardSlot { //TODO: this could be an option as well?
    Empty,
    Card, //TODO: this is not working as expected
}
*/

/*
impl CardSlot {
    pub fn expect_borrow(&self) -> &Card {
        match self {
            &CardSlot::Empty => panic!("You are trying to borrow a non-dealt card"),
            x @ &CardSlot::Card => x
        }
    }
}
*/

/*
pub fn expectCard<'a>(cs: CardSlot) -> &'a Card {
    match cs {
        None => panic!("You are trying to borrow a non-dealt card"),
        Some(ref x) => x
    }
}
*/

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
