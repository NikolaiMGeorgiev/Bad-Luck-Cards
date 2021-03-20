use bad_luck_cards::computer::Computer;
use bad_luck_cards::player::Player;
use bad_luck_cards::card::*;
use bad_luck_cards::deck::Deck;

#[test]
fn test_computer_choose_contract(){
    let mut computer = Computer::new();
    let mut contract = Contract::Pass;

    computer.cards = vec![Card::new(String::from("A"), String::from("♤")), Card::new(String::from("10"), String::from("♣")),
                    Card::new(String::from("J"), String::from("♣")), Card::new(String::from("9"), String::from("♣")),
                    Card::new(String::from("7"), String::from("♣"))];

    //Test suit (e.g. Spades)
    computer.choose_contract(&mut contract);
    assert_eq!(Deck::get_contract_weight(&contract), 1);

    //Test all trumps
    contract = Contract::Hearts;

    computer.cards.remove(3);
    computer.cards.push(Card::new(String::from("J"), String::from("♢")));

    computer.choose_contract(&mut contract);
    assert_eq!(Deck::get_contract_weight(&contract), 6);

    //Test no trumps
    contract = Contract::Hearts;

    computer.cards.pop();
    computer.cards.push(Card::new(String::from("10"), String::from("♢")));

    computer.choose_contract(&mut contract);
    assert_eq!(Deck::get_contract_weight(&contract), 5);

    //Test pass
    contract = Contract::Pass;

    computer.cards = vec![Card::new(String::from("8"), String::from("♤")), Card::new(String::from("10"), String::from("♤")),
                    Card::new(String::from("8"), String::from("♣")), Card::new(String::from("9"), String::from("♣")),
                    Card::new(String::from("7"), String::from("♣"))];

    computer.choose_contract(&mut contract);
    assert_eq!(Deck::get_contract_weight(&contract), 0);
}

#[test]
fn test_computer_make_first_turn(){
    let mut computer = Computer::new();
    let mut cards = Vec::new();

    computer.cards.push(Card::new(String::from("A"), String::from("♤")));
    computer.cards.push(Card::new(String::from("10"), String::from("♣")));
    computer.cards.push(Card::new(String::from("J"), String::from("♣")));

    let mut result = computer.make_turn(&cards, &Contract::AllDonaldTrumps);
    assert_eq!(result, 2);

    result = computer.make_turn(&cards, &Contract::NoDonaldTrumps);
    assert_eq!(result, 0);

    result = computer.make_turn(&cards, &Contract::Clubs);
    assert_eq!(result, 2);
}

#[test]
fn test_computer_make_second_turn(){
    let mut computer = Computer::new();
    let mut cards = vec![Card::new(String::from("9"), String::from("♣"))];

    computer.cards.push(Card::new(String::from("A"), String::from("♤")));
    computer.cards.push(Card::new(String::from("10"), String::from("♣")));
    computer.cards.push(Card::new(String::from("J"), String::from("♣")));

    let mut result = computer.make_turn(&cards, &Contract::AllDonaldTrumps);
    assert_eq!(result, 2);

    result = computer.make_turn(&cards, &Contract::NoDonaldTrumps);
    assert_eq!(result, 1);

    result = computer.make_turn(&cards, &Contract::Clubs);
    assert_eq!(result, 2);

    cards = vec![Card::new(String::from("9"), String::from("♥"))];

    result = computer.make_turn(&cards, &Contract::Spades);
    assert_eq!(result, 0);
}

#[test]
fn test_computer_make_third_fourth_turn(){
    let mut computer = Computer::new();
    let mut cards = vec![Card::new(String::from("8"), String::from("♣")), Card::new(String::from("9"), String::from("♣")), 
                        Card::new(String::from("K"), String::from("♤"))];

    computer.cards.push(Card::new(String::from("A"), String::from("♤")));
    computer.cards.push(Card::new(String::from("10"), String::from("♣")));
    computer.cards.push(Card::new(String::from("J"), String::from("♣")));

    let mut result = computer.make_turn(&cards, &Contract::AllDonaldTrumps);
    assert_eq!(result, 2);

    result = computer.make_turn(&cards, &Contract::NoDonaldTrumps);
    assert_eq!(result, 1);

    result = computer.make_turn(&cards, &Contract::Spades);
    assert_eq!(result, 0);

    cards = vec![Card::new(String::from("8"), String::from("♥")), Card::new(String::from("9"), String::from("♥")), 
                Card::new(String::from("K"), String::from("♥"))];
    
    result = computer.make_turn(&cards, &Contract::Spades);
    assert_eq!(result, 0);
}