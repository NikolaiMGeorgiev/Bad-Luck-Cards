use bad_luck_cards::deck::Deck;
use bad_luck_cards::card::*;

#[test]
fn test_create_card(){
    let card = Card::new(String::from("A"), String::from("♢"));

    assert_eq!(card.rank, String::from("A"));
    assert_eq!(card.suit, String::from("♢"));
}

#[test]
fn test_create_deck(){
    let deck = Deck::new();

    assert_eq!(deck.cards_left.len(), 32);
    assert_eq!(deck.cards_left[0].rank, String::from("A"));
    assert_eq!(deck.cards_left[0].suit, String::from("♢"));
}

#[test]
fn test_deck_shuffle(){
    //Might fail since it's possible for the first element to stay the same
    let mut deck = Deck::new();
    let card = deck.cards_left[0].display();

    deck.shuffle();

    assert_ne!(deck.cards_left[0].display(), card);
}