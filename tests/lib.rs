extern crate holdem;
extern crate cards;

use holdem::{CardSlot};
use cards::card::{Card, Value, Suit};

#[test]
fn borrow_card() {
    let card_slot = CardSlot::Dealt(Card(Value::Two, Suit::Hearts));
    let card = card_slot.expect_borrow();
    assert_eq!(card, &Card(Value::Two, Suit::Hearts));
}

#[test]
#[should_panic]
fn borrow_card_fail() {
    let card_slot = CardSlot::Empty;
    let _card = card_slot.expect_borrow();
}
