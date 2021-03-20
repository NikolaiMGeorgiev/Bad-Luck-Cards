extern crate rand;
use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::card::*;

pub struct Deck{
    pub cards_left: Vec<Card>
}

impl Deck{
    pub fn new() -> Deck{
        Deck{
            cards_left: vec![
                Card::new(String::from("A"), String::from("♢")), Card::new(String::from("10"), String::from("♢")), 
                Card::new(String::from("K"), String::from("♢")), Card::new(String::from("Q"), String::from("♢")), 
                Card::new(String::from("J"), String::from("♢")), Card::new(String::from("9"), String::from("♢")), 
                Card::new(String::from("8"), String::from("♢")), Card::new(String::from("7"), String::from("♢")),
                Card::new(String::from("A"), String::from("♣")), Card::new(String::from("10"), String::from("♣")), 
                Card::new(String::from("K"), String::from("♣")), Card::new(String::from("Q"), String::from("♣")), 
                Card::new(String::from("J"), String::from("♣")), Card::new(String::from("9"), String::from("♣")), 
                Card::new(String::from("8"), String::from("♣")), Card::new(String::from("7"), String::from("♣")),
                Card::new(String::from("A"), String::from("♤")), Card::new(String::from("10"), String::from("♤")), 
                Card::new(String::from("K"), String::from("♤")), Card::new(String::from("Q"), String::from("♤")), 
                Card::new(String::from("J"), String::from("♤")), Card::new(String::from("9"), String::from("♤")), 
                Card::new(String::from("8"), String::from("♤")), Card::new(String::from("7"), String::from("♤")),
                Card::new(String::from("A"), String::from("♥")), Card::new(String::from("10"), String::from("♥")), 
                Card::new(String::from("K"), String::from("♥")), Card::new(String::from("Q"), String::from("♥")), 
                Card::new(String::from("J"), String::from("♥")), Card::new(String::from("9"), String::from("♥")), 
                Card::new(String::from("8"), String::from("♥")), Card::new(String::from("7"), String::from("♥"))
            ]
        }
    }

    pub fn shuffle(&mut self){
        self.cards_left.shuffle(&mut thread_rng())
    }

    pub fn get_contract_weight(contract: &Contract) -> u8{
        match contract{
            Contract::AllDonaldTrumps => 6,
            Contract::NoDonaldTrumps => 5,
            Contract::Spades => 4,
            Contract::Hearts => 3,
            Contract::Diamonds => 2,
            Contract::Clubs => 1,
            Contract::Pass => 0
        }
    }

    pub fn contract_to_str(contract: &Contract) -> &str{
        match contract{
            Contract::AllDonaldTrumps => "all",
            Contract::NoDonaldTrumps => "no",
            Contract::Spades => "♤",
            Contract::Hearts => "♥",
            Contract::Diamonds => "♢",
            Contract::Clubs => "♣",
            Contract::Pass => "pass"
        }
    }
}