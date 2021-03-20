extern crate rand;

use bad_luck_cards::game_match::*;

fn main(){
    let mut game = GameMatch::new();

    game.setup_game();

    game.play();
}
