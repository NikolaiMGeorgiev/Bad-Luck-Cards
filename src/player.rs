extern crate rand;
use std::io::{self, BufRead};

use crate::card::*;

pub struct Player{
    pub cards: Vec<Card>,
    pub points: u8
}

impl Player{
    pub fn new() -> Player{
        Player{
            cards: Vec::new(),
            points: 0
        }
    }

    pub fn display_cards(&self){
        for card in &self.cards{
            print!("|{}| ", &card.display());
        }
        println!();
    }

    pub fn display_cards_as_str(&self) -> String{
        let mut result= "".to_owned();
        for card in &self.cards{
            result = format!("{}|{}|", result.to_owned(), &card.display().to_owned());
        }

        result
    }

    pub fn make_turn(&mut self) -> Card{
        println!("Choose card: ");
        let mut index = 0;
        let stdin = io::stdin();
        
        for line in stdin.lock().lines(){
            match line.unwrap().parse::<usize>(){
                Ok(number) => {
                    index = number;

                    if index >= 0 && index < self.cards.len() {
                        return self.cards.remove(index);
                    }else{
                        println!("Wrong input. Please enter the index of the card.");
                    }
                },
                Err(e) => {
                    println!("Wrong input. Please enter the index of the card.");
                    continue;
                }
            }
        }

        self.cards.remove(index)
    }
}