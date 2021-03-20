extern crate rand;

use crate::card::*;
use crate::deck::*;

pub struct Computer{
    pub cards: Vec<Card>,
    pub points: u8
}

impl Computer{
    pub fn new() -> Computer{
        Computer{
            cards: Vec::new(),
            points: 0
        }
    }

    fn get_suit_index(card: &Card) -> usize{
        if card.suit == "♣" {
            0
        }else if card.suit == "♢" {
            1
        }else if card.suit == "♥" {
            2
        }else{
            3
        }
    }

    fn check_good_suit(cards_count: u8, suit_index: usize, jokers: &Vec<u8>, nines: &Vec<u8>, aces: &Vec<u8>, tens: &Vec<u8>) -> bool{
        if (jokers[suit_index] == 1 && nines[suit_index] == 1) || (jokers[suit_index] == 1 && aces[suit_index] == 1 && cards_count > 2) || 
                (nines[suit_index] == 1 && (aces [suit_index] == 1) || cards_count >= 4){
            true
        }else{
            false
        }
    }

    pub fn choose_contract(&self, contract: &mut Contract) {
        let contr_weight = Deck::get_contract_weight(contract);
        let mut spades_count = 0;
        let mut hearts_count = 0;
        let mut diamonds_count = 0;
        let mut clubs_count = 0;
        let mut suit_index;
        let mut aces_count = 0;
        let mut tens_count = 0;
        let mut jokers_count = 0;
        let mut nines_count = 0;

        //let mut _ = vec![x_of_clubs, x_of_diamonds, x_of_hearts, x_of_spades]
        let mut aces = vec![0, 0, 0, 0];
        let mut tens = vec![0, 0, 0, 0];
        let mut jokers = vec![0, 0, 0, 0];
        let mut nines = vec![0, 0, 0, 0];

        if contr_weight == 6 {
            return
        }

        for card in &self.cards{
            suit_index = Computer::get_suit_index(&card);
            if card.rank == "J"{
                jokers[suit_index] = 1;
                jokers_count += 1;
            }else if card.rank == "9"{
                nines[suit_index] = 1;
                nines_count += 1;
            }else if card.rank == "A"{
                aces[suit_index] = 1;
                aces_count += 1;
            }else if card.rank == "10"{
                tens[suit_index] = 1;
                tens_count += 1;
            }

            if card.suit == "♣"{
                clubs_count += 1;
            }else if card.suit == "♢"{
                diamonds_count += 1;
            }else if card.suit == "♥"{
                hearts_count += 1;
            }else {
                spades_count += 1;
            }
        }

        if jokers_count >= 2 || (jokers_count == 1 && nines_count >= 2){
            *contract = Contract::AllDonaldTrumps;
        }else if contr_weight < 5 && (aces_count >= 2 || (aces_count == 1 && tens_count >= 2)){
            *contract = Contract::NoDonaldTrumps;
        }else if contr_weight < 4 && spades_count >= 2 && Computer::check_good_suit(spades_count, 0, &jokers, &nines, &aces, &tens){
            *contract = Contract::Spades;
        }else if contr_weight < 3 && hearts_count >= 2 && Computer::check_good_suit(hearts_count, 1, &jokers, &nines, &aces, &tens){
            *contract = Contract::Hearts;
        }else if contr_weight < 2 && diamonds_count >= 2 && Computer::check_good_suit(diamonds_count, 2, &jokers, &nines, &aces, &tens){
            *contract = Contract::Diamonds;
        }else if contr_weight < 1 && clubs_count >= 2 && Computer::check_good_suit(clubs_count, 3, &jokers, &nines, &aces, &tens){
            *contract = Contract::Clubs;
        }
    }

    pub fn make_turn(&self, cards: &Vec<Card>, contract: &Contract) -> usize{
        let mut index;

        if cards.len() == 0{
            self.choose_first_card(contract)
        }else{
            if cards.len() == 1{
                index = self.choose_second_card(cards, contract);
            }else {
                index = self.choose_third_fourth_card(cards, contract);
            }

            if index != 10{
                println!("card on index {}",index);
                index
            }else{
                self.choose_worst_card(contract)
            }
        }
    }

    fn choose_first_card(&self, contract: &Contract) -> usize{
        let mut index = 0;

        for i in 0..self.cards.len(){
            if self.cards[i].get_card_weight(contract) > self.cards[index].get_card_weight(contract){
                index = i;
            }
        }

        index
    }

    fn choose_second_card(&self, cards: &Vec<Card>, contract: &Contract) -> usize{
        let contr = Deck::contract_to_str(contract);

        //Look for stronger card from the same suit as cards[0].
        for i in 0..self.cards.len(){
            if self.cards[i].suit == cards[0].suit && 
                self.cards[i].get_card_weight(contract) > cards[0].get_card_weight(contract){
                return i;
            }
        }

        //No stronger cards from the same suit as cards[0] were found.
        //If cards[0] was from the contract suit, then we can give any card. Therefore, we check the alternatives.
        
        //Try to find a card for "kozene" if contract isn't all trumps or no trumps and cards[0] isn't the same suit as cnontract.
        if cards[0].suit != contr && Deck::get_contract_weight(contract) >= 1 && Deck::get_contract_weight(contract) <= 4{
            for i in 0..self.cards.len(){
                if self.cards[i].suit == contr.to_string(){
                    return i;
                }
            }
        }

        //cards[0] was from the contract suit or contract is all trumps or no trumps
        10
    }

    fn choose_third_fourth_card(&self, cards: &Vec<Card>, contract: &Contract) -> usize{
        let mut strongest_card_index = 0;
        let mut contract_card_index = 10;
        let contr = Deck::contract_to_str(contract).to_string();

        //Check if there has been a "kozene". If so find the strongest contract card on the table.
        for i in 0..cards.len(){
            if cards[i].suit == contr {
                if contract_card_index != 10 && 
                    cards[i].get_card_weight(contract) > cards[contract_card_index].get_card_weight(contract){
                    contract_card_index = i;
                }else{
                    contract_card_index = i;
                }
            }
        }

        //There has been a "kozene". Find a stronger contract card.
        if contract_card_index != 10 {
            for i in 0..self.cards.len(){
                if self.cards[i].suit == contr &&
                self.cards[i].get_card_weight(contract) > cards[contract_card_index].get_card_weight(contract){
                    return i;
                }
            }
        }else{
            //There hasn't been a "kozene".

            //Find strongest card on table.
            for i in 0..cards.len(){
                if cards[i].suit == cards[0].suit &&
                cards[i].get_card_weight(contract) > cards[strongest_card_index].get_card_weight(contract){
                    strongest_card_index = i;
                }
            }
    
            //Try to find a card matching cards[0]'s suit that is stronger than all card on the table.
            for i in 0..self.cards.len(){
                if self.cards[i].suit == cards[0].suit &&
                self.cards[i].get_card_weight(contract) > cards[strongest_card_index].get_card_weight(contract){
                    return i;
                }
            }
            
            //No stronger card from cards[0]'s suit has been found. 
            //Try to perform "kozene" if contract is a suit.
            if Deck::get_contract_weight(contract) >= 1 && Deck::get_contract_weight(contract) <= 4 {
                for i in 0..self.cards.len(){
                    if self.cards[i].suit == contr {
                        return i;
                    }
                }
            }
        }

        //No cards for "kozene" or (contract is all/no trumps and no bigger cards from cards[0]'s suit were found).
        10
    }

    fn choose_worst_card(&self, contract: &Contract) -> usize{
        let mut card_index = 0;

        for i in 0..self.cards.len(){
            if self.cards[i].get_card_weight(contract) < self.cards[card_index].get_card_weight(contract){
                card_index = i;
            }
        }

        card_index
    }

    pub fn display_cards(&self){
        for card in &self.cards{
            print!("|{}| ", &card.display());
        }
        println!();
    }

    pub fn display_cards_as_str(&self) -> String{
        let mut result = "".to_owned();
        for card in &self.cards{
            result = format!("{}|{}|", result.to_owned(), &card.display().to_owned());
        }

        result
    }
}