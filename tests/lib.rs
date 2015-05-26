extern crate holdem;
extern crate cards;

use holdem::{HandRankClass, hand_rank_to_class};

#[test]
fn ordering_of_handrankclass() {
    assert!(HandRankClass::HighCard < HandRankClass::FullHouse);
    assert!(HandRankClass::StraightFlush == HandRankClass::StraightFlush);
}

#[test]
fn hand_rank_to_class_conversion() {
   assert_eq!(hand_rank_to_class(&0), HandRankClass::HighCard); // the worst hand rank
   assert_eq!(hand_rank_to_class(&1276), HandRankClass::HighCard);
   assert_eq!(hand_rank_to_class(&1277), HandRankClass::OnePair);
   assert_eq!(hand_rank_to_class(&(1277+2860)), HandRankClass::TwoPair);
   assert_eq!(hand_rank_to_class(&7451), HandRankClass::FourOfAKind);
   assert_eq!(hand_rank_to_class(&7461), HandRankClass::StraightFlush); // the best hand rank
}

#[test]
#[should_panic]
fn too_large_hand_rank() {
    hand_rank_to_class(&7462);
}
