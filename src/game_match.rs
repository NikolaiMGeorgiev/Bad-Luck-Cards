extern crate rand;
use rand::Rng;
use std::io::{self, BufRead};

use crate::player::*;
use crate::computer::*;
use crate::card::*;
use crate::deck::*;

pub struct GameMatch{
    pub deck: Deck,
    pub player: Player,
    pub computers: Vec<Computer>,
    pub current_cards: Vec<Card>,
    pub contract: Contract,
    pub punishments: Vec<String>
}

impl GameMatch{
    pub fn new() -> GameMatch{
        GameMatch{
            deck: Deck::new(),
            player: Player::new(),
            computers: vec![Computer::new(), Computer::new(), Computer::new()],
            current_cards: Vec::new(),
            contract: Contract::Pass,
            punishments: vec![
                String::from("Your teammate has been spending too much time on social media. Now they thinks all their friend are fake and 
are using them. Therefore they use their worst card to pay you back."),
                String::from("Oh no! Is this an earthquake? Or just a little green troll underneath the table is shaking it? 
Doesn't matter becasue the cards from last round are gone and with them are gone their points."),
                String::from("An ancient family curse is finally revelaing itself to you. That's just not RIGHT! Therefore, 
the rightmost card of your hand is transformed into a ♣7. Worry not! Clovers are believed to bring good luck."),
                String::from("You got drunk. Drunk on pride. And when you're drunk, you're sloppy. So you drop a random card from your hand."),
                String::from("Look, what's that behind you? It's your enemy stealing a card from you while not looking, that's what it is."),
                String::from("Hero, hero, here you go! Too bad your points are back to zero.")
            ]
        }
    }

    pub fn play(&mut self){
        //Legend: 0 = computers[0], 1 = computers[1], 2 = computers[2], 3 = player
        let mut first_player = 3;
        let mut last_winner = 3;
        let mut card_index = 0;
        let mut has_angry_teammate = false;
        let mut has_sloppy_card = false;
        let mut angry_teammate = 3;
        let mut winner = 3;

        match self.contract{
            Contract::Pass => return,
            _ => ()
        }

        for i in 0..8{
            //Check for sloppy card punishment
            if self.current_cards.len() == 4{
                self.current_cards = Vec::new();
            }else if self.current_cards.len() == 1{
                //first_player already played their sloppy card so we start from the next player.
                first_player = (first_player + 1) % 4;
                has_sloppy_card = true;
            }

            self.print_screen(first_player);

            //Check for angry teammate punishment
            if has_angry_teammate{
                angry_teammate = (winner + 2) % 4;
            }
            
            //If the sloppy card was played by Player (with index 3) this for loop will make Player play another card. 
            //Therefore, we add first_player != 0
            if has_sloppy_card == false || (has_sloppy_card && first_player != 0){
                for j in first_player..4{
                    //Check if j is the angry teammate and give them a massage
                    if has_angry_teammate && j == angry_teammate{
                        self.let_anger_out(angry_teammate);
                        has_angry_teammate = false;
    
                        self.print_screen(first_player);
                        continue;
                    }
    
                    if j != 3{
                        card_index = self.computers[j].make_turn(&self.current_cards, &self.contract);
                        self.current_cards.push(self.computers[j].cards.remove(card_index));
                    }else{
                        self.current_cards.push(self.player.make_turn());
                    }
    
                    self.print_screen(first_player);
                }
            }

            //In the begining of the for loop we set first_player as the player next to first player.
            //Therefore, now we need to reset first_player to its original value.
            if has_sloppy_card{
                first_player = (first_player + 3) % 4;
                has_sloppy_card = false;
            }

            for j in 0..first_player{
                if has_angry_teammate && j == angry_teammate{
                    self.let_anger_out(angry_teammate);
                    has_angry_teammate = false;

                    self.print_screen(first_player);
                    continue;
                }

                if j != 3{
                    card_index = self.computers[j].make_turn(&self.current_cards, &self.contract);
                    self.current_cards.push(self.computers[j].cards.remove(card_index));
                }else{
                    self.current_cards.push(self.player.make_turn());
                }

                self.print_screen(first_player);
            }

            card_index = 0;

            //Find the winning card's position and save it in card_index.
            for j in 0..self.current_cards.len(){
                if self.current_cards[j].get_card_weight(&self.contract) > self.current_cards[card_index].get_card_weight(&self.contract){
                    card_index = j;
                }
            }

            //Make the new winner the player who played the winning card at index card_index.
            winner = (last_winner + card_index) % 4;
            let mut points = 0;

            for j in 0..self.current_cards.len(){
                points += self.current_cards[j].get_card_score(&self.contract);
            }

            if winner != 3{
                self.computers[winner].points += points;
            }else{
                self.player.points += points;
            }

            //In the first round winner is always equal to last_winner(= 3). Therefore, we need i != 0, where i is round number.
            //execute_punishment shouldn't be called the last turn since some punishments involve actions with cards. Hence we need i != 7.
            if winner == last_winner && i != 0 && i != 7{
                self.execute_punishment(GameMatch::roll_dice(), winner, points, &mut has_angry_teammate);
            }

            first_player = winner;
            last_winner = winner;
            
            self.print_screen(first_player);

            println!("\nPlayer points: {}", self.player.points);
            for j in 0.. 3{
                println!("Comp {} points: {}", j, self.computers[j].points);
            }
        }

        if self.computers[0].points + self.computers[2].points > self.computers[1].points + self.player.points{
            println!("You lose!\nWinners: Computer 0 and Computer 2 with {} points\nLosers: Computer 1 and Player with {} points",
                    self.computers[0].points + self.computers[2].points, self.computers[1].points + self.player.points);
        }else if self.computers[0].points + self.computers[2].points < self.computers[1].points + self.player.points{
            println!("You win!\nWinners: Computer 1 and Player with {} points\nLosers: Computer 0 and Computer 2 with {} points",
                    self.computers[1].points + self.player.points, self.computers[0].points + self.computers[2].points);
        }else{
            println!("Draw! You all suck...");
        }
    }

    pub fn display_cards_as_str(&self) -> String{
        let mut result = "".to_owned();

        for card in &self.current_cards{
            result = format!("{}|{}|", result.to_owned(), &card.display().to_owned());
        }

        result
    }

    fn print_screen(&mut self, first_player: usize){
        let stdin = io::stdin();

        println!("Press any key to continue...");
        for line in stdin.lock().lines(){
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            break;
        }

        println!("{}", format!("{:-^107}", "BAD LUCK CARDS"));
        println!("Contract: {:?}", self.contract);
        println!("|{}|", format!("{:-<105}", ""));
        println!("|{}|", format!("{:^105}", "Computer 1"));
        println!("|{}|", format!("{:<35}{:^35}{:<35}", "", self.computers[1].display_cards_as_str(), ""));
        println!("|{}|", format!("{:<105}", ""));
        println!("|{}|", format!("{:^35}{:^35}{:^35}", "Computer 0", "", "Computer 2"));
        println!("|{}|", format!("{:^35}{:^35}{:^35}", self.computers[0].display_cards_as_str(), self.display_cards_as_str(), self.computers[2].display_cards_as_str()));
        println!("|{}|", format!("{:<105}", ""));
        println!("|{}|", format!("{:<105}", ""));
        println!("|{}|", format!("{:<35}{:^35}{:<35}", "", self.player.display_cards_as_str(), ""));
        println!("|{}|", format!("{:^105}", "Player"));
        println!("|{}|", format!("{:-<105}", ""));

        println!("Current turn: {}", first_player);
        println!("{}\n", format!("{:-<107}", ""));
    }

    pub fn roll_dice() -> u8{
        let mut rng = rand::thread_rng();
        rng.gen_range(1..7)
    }

    pub fn execute_punishment(&mut self, number: u8, winner: usize, points: u8, has_angry_teammate: &mut bool){
        if number == 1{
            *has_angry_teammate = true;
        }else if number == 2{
            self.take_points(winner, points);
        }else if number == 3{
            self.transform_rightmost(winner)
        }else if number == 4{
            self.play_sloppy_card(winner);
        }else if number == 5{
            self.swap_cards(winner);
        }else if number == 6{
            self.points_to_zero(winner);
        }

        println!("\nMultiple wins in a roll. Time for punishment: ");
        println!("{}\n", self.punishments[usize::from(number) - 1]);
    }

    fn let_anger_out(&mut self, teammate: usize){
        let mut worst_card = 0;

        if teammate != 3{
            for i in 0..self.computers[teammate].cards.len(){
                if self.computers[teammate].cards[i].get_card_weight(&self.contract) < 
                self.computers[teammate].cards[worst_card].get_card_weight(&self.contract){
                    worst_card = i;
                }
            }

            self.current_cards.push(self.computers[teammate].cards.remove(worst_card));
        }else{
            for i in 0..self.player.cards.len(){
                if self.player.cards[i].get_card_weight(&self.contract) < self.player.cards[worst_card].get_card_weight(&self.contract){
                        worst_card = i;
                }
            }

            self.current_cards.push(self.player.cards.remove(worst_card));
        }
    }

    fn take_points(&mut self, winner: usize, points: u8){
        if winner != 3{
            println!("Computer {}'s points before: {}", winner, self.computers[winner].points);
            self.computers[winner].points -= points;
            println!("Computer {}'s points after: {}", winner, self.computers[winner].points);
        }else{
            println!("Player's points before: {}", self.player.points);
            self.player.points -= points;
            println!("Player's points after: {}", self.player.points);
        }
    }

    fn transform_rightmost(&mut self, winner: usize){
        let mut n;

        if winner != 3{
            println!("Computer {}'s normal cards:", winner);
            self.computers[winner].display_cards();

            n = self.computers[winner].cards.len();
            self.computers[winner].cards[n - 1].suit = "♣".to_string();
            self.computers[winner].cards[n - 1].rank = "7".to_string();

            println!("Computer {}'s changed cards:", winner);
            self.computers[winner].display_cards();
        }else{
            println!("Player's normal cards:");
            self.player.display_cards();

            n = self.player.cards.len();
            self.player.cards[n - 1].suit = "♣".to_string();
            self.player.cards[n - 1].rank = "7".to_string();

            println!("Player's changed cards:");
            self.player.display_cards();
        }
    }

    fn play_sloppy_card(&mut self, winner: usize){
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.player.cards.len());
        self.current_cards = Vec::new();
        
        if winner != 3{
            println!("Computer {} plays sloppy card: {:?}", winner, self.computers[winner].cards[index]);
            self.current_cards.push(self.computers[winner].cards.remove(index));
        }else{
            println!("Player plays sloppy card: {:?}", self.player.cards[index]);
            self.current_cards.push(self.player.cards.remove(index));
        }
    }

    fn swap_cards(&mut self, winner: usize){
        let mut own_card = 0;
        let mut own_suit;
        let mut enemy_card = 0;
        let mut own_rank;
        let mut win = winner;
        let mut is_first_line = true;
        let stdin = io::stdin();

        if winner != 3{
            println!("Choose card index to steal:");

            for line in stdin.lock().lines(){
                match line.unwrap().parse::<usize>(){
                    Ok(index) => {
                        if index >= 0 && index < self.player.cards.len() {
                            if is_first_line{
                                enemy_card = index;
                                is_first_line = false;
                            }else{
                                own_card = index;
                                break;
                            }
                        }else{
                            println!("Wrong input. Please enter the index of the card to steal.");
                        }
                    },
                    Err(e) => {
                        println!("Wrong input. Please enter the index of the card to steal.");
                    }
                }

                if is_first_line{
                    println!("Choose card index to steal:");
                }else{
                    println!("Choose own card index to swap:");
                }
            }
        }else{
            let mut rng = rand::thread_rng();
            win = winner - 1;
            enemy_card = rng.gen_range(0..self.computers[win].cards.len());
            own_card = rng.gen_range(0..self.player.cards.len());
        }

        println!("\nBefore swap: ");
        println!("Player cards:");
        self.player.display_cards();
        println!("Computer {}'s cards:", win);
        self.computers[win].display_cards();

        own_rank = self.player.cards[own_card].rank.clone();
        own_suit = self.player.cards[own_card].suit.clone();
        
        self.player.cards[own_card].suit = self.computers[win].cards[enemy_card].suit.clone();
        self.player.cards[own_card].rank = self.computers[win].cards[enemy_card].rank.clone();
        self.computers[win].cards[enemy_card].rank = own_rank;
        self.computers[win].cards[enemy_card].suit = own_suit;

        println!("\nAfter swap: ");
        println!("Player cards:");
        self.player.display_cards();
        println!("Computer {}'s cards:", win);
        self.computers[win].display_cards();
    }

    fn points_to_zero(&mut self, winner: usize){
        if winner != 3{
            println!("Computer {}'s points before: {}", winner, self.computers[winner].points);
            self.computers[winner].points = 0;
            println!("Computer {}'s points after: {}", winner, self.computers[winner].points);
        }else{
            println!("Player's points before: {}", self.player.points);
            self.player.points = 0;
            println!("Player's points after: {}", self.player.points);
        }
    }

    fn setup_contract(&mut self){
        let mut passes = 0;
        let stdin = io::stdin();
        let mut current_contract = self.contract;

        println!("Choose contract (c = Clubs, d = Diamonds, h = Heart, s = Spades, n = No Trumps, a = All trumps):");
        for line in stdin.lock().lines() {
            let player_contract = line.unwrap();

            if player_contract != "p" && player_contract != "c" && player_contract != "d" && player_contract != "h" && 
                player_contract != "s" && player_contract != "n" && player_contract != "a"{
                println!("Wrong input! Please use one of the following: c, d, h, s, n, a");
                continue;
            }
            
            passes = 0;

            if player_contract == "p" {
                passes += 1;
            }else if player_contract == "c" && Deck::get_contract_weight(&self.contract) < 1{
                self.contract = Contract::Clubs;
            }else if player_contract == "d" && Deck::get_contract_weight(&self.contract) < 2{
                self.contract = Contract::Diamonds;
            }else if player_contract == "h" && Deck::get_contract_weight(&self.contract) < 3{
                self.contract = Contract::Hearts;
            }else if player_contract == "s" && Deck::get_contract_weight(&self.contract) < 4{
                self.contract = Contract::Spades;
            }else if player_contract == "n" && Deck::get_contract_weight(&self.contract) < 5{
                self.contract = Contract::NoDonaldTrumps;
            }else if player_contract == "a" && Deck::get_contract_weight(&self.contract) < 6{
                self.contract = Contract::AllDonaldTrumps;
            }else{
                println!("Please chose higher suit or pass!");
                continue;
            }

            current_contract = self.contract;

            for i in 0..3{
                &self.computers[i].choose_contract(&mut self.contract);

                if Deck::get_contract_weight(&current_contract) == Deck::get_contract_weight(&self.contract){
                    passes += 1; 
                    println!("Computer{} says: {:?}", i, Contract::Pass);
                }else{
                    current_contract = self.contract;
                    println!("Computer{} says: {:?}", i, current_contract);
                }
            }

            if passes == 4{
                return
            }

            println!("\n\nCurrent contract:{:?}", self.contract);
            println!("Choose contract (c = Clubs, d = Diamonds, h = Heart, s = Spades, n = No Trumps, a = All trumps):");
        }
    }

    pub fn setup_game(&mut self){
        self.deck.shuffle();

        for i in 0..3{
            self.player.cards.push(self.deck.cards_left.pop().unwrap());
        }

        for j in 0..3{
            for i in 0..3{
                self.computers[j].cards.push(self.deck.cards_left.pop().unwrap());
            }
        }

        for i in 0..2{
            self.player.cards.push(self.deck.cards_left.pop().unwrap());
        }

        for j in 0..3{
            for i in 0..2{
                self.computers[j].cards.push(self.deck.cards_left.pop().unwrap());
            }
        }

        self.print_screen(3);

        self.setup_contract();
        println!("\nContract chosen: {:?}", self.contract);

        match self.contract{
            Contract::Pass => return,
            _ => ()
        }

        for i in 0..3{
            self.player.cards.push(self.deck.cards_left.pop().unwrap());
        }

        for j in 0..3{
            for i in 0..3{
                self.computers[j].cards.push(self.deck.cards_left.pop().unwrap());
            }
        }
    }
}