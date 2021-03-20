extern crate rand;

use crate::deck::*;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum Contract {
    AllDonaldTrumps,
    NoDonaldTrumps,
    Spades, //♤
    Hearts, //♥
    Diamonds, //♢
    Clubs, //♣
    Pass
}

#[derive(Debug)]
pub struct Card{
    pub rank: String,
    pub suit: String
}

impl Card{
    pub fn new(rank: String, suit: String) -> Card{
        Card{
            rank,
            suit
        }
    }

    pub fn get_card_weight(&self, contract: &Contract) -> u8{
        let contr_to_str = Deck::contract_to_str(contract);

        if self.suit == contr_to_str{
            if "J".to_string() == self.rank { return 16 }
            else if "9".to_string() == self.rank { return 15 }
            else if "A".to_string() == self.rank { return 14 }
            else if "10".to_string() == self.rank { return 13 }
            else if "K".to_string() == self.rank { return 12 }
            else if "Q".to_string() == self.rank { return 11 }
            else if "8".to_string() == self.rank { return 10 }
            else if "7".to_string() == self.rank { return 9 }
        }else if contr_to_str == "all"{
            if "J".to_string() == self.rank { return 16 }
            else if "9".to_string() == self.rank { return 15 }
            else if "A".to_string() == self.rank { return 14 }
            else if "10".to_string() == self.rank { return 13 }
            else if "K".to_string() == self.rank { return 12 }
            else if "Q".to_string() == self.rank { return 11 }
            else if "8".to_string() == self.rank { return 0 }
            else if "7".to_string() == self.rank { return 0 }
        }else if self.suit != contr_to_str || contr_to_str == "no"{
            if "A".to_string() == self.rank { return 6 }
            else if "10".to_string() == self.rank { return 5 }
            else if "K".to_string() == self.rank { return 4 }
            else if "Q".to_string() == self.rank { return 3 }
            else if "J".to_string() == self.rank { return 2 }
            else if "9".to_string() == self.rank { return 0 }
            else if "8".to_string() == self.rank { return 0 }
            else if "7".to_string() == self.rank { return 0 }
        }

        0
    }

    pub fn get_card_score(&self, contract: &Contract) -> u8{
        let contr_to_str = Deck::contract_to_str(contract);

        if self.suit == contr_to_str || contr_to_str == "all"{
            if "J".to_string() == self.rank { return 20 }
            else if "9".to_string() == self.rank { return 14 }
            else if "A".to_string() == self.rank { return 11 }
            else if "10".to_string() == self.rank { return 10 }
            else if "K".to_string() == self.rank { return 4 }
            else if "Q".to_string() == self.rank { return 3 }
            else if "8".to_string() == self.rank { return 0 }
            else if "7".to_string() == self.rank { return 0 }
        }else if self.suit != contr_to_str || contr_to_str == "no"{
            if "A".to_string() == self.rank { return 11 }
            else if "10".to_string() == self.rank { return 10 }
            else if "K".to_string() == self.rank { return 4 }
            else if "Q".to_string() == self.rank { return 3 }
            else if "J".to_string() == self.rank { return 2 }
            else if "9".to_string() == self.rank { return 0 }
            else if "8".to_string() == self.rank { return 0 }
            else if "7".to_string() == self.rank { return 0 }
        }

        0
    }

    pub fn display(&self) -> String{
        format!("{}{}", self.rank, self.suit)
    }
}