use bad_luck_cards::game_match::GameMatch;
use bad_luck_cards::card::Card;

#[test]
fn test_punishment_init(){
    let game = GameMatch::new();

    assert_eq!(game.punishments.len(), 6);
}

#[test]
fn test_has_angry_teammate(){
    let mut game = GameMatch::new();
    let mut has_angry_teammate = false;

    game.execute_punishment(1, 0, 5, &mut has_angry_teammate);

    assert_eq!(has_angry_teammate, true);
}

#[test]
fn test_take_points(){
    let mut game = GameMatch::new();

    game.computers[0].points = 15;
    game.player.points = 15;

    game.execute_punishment(2, 0, 5, &mut false);
    game.execute_punishment(2, 3, 5, &mut false);

    assert_eq!(game.computers[0].points, 10);
    assert_eq!(game.player.points, 10);
}


#[test]
fn test_transform_rightmost(){
    let mut game = GameMatch::new();

    game.computers[0].cards.push(Card::new(String::from("A"),String::from("♢")));
    game.computers[0].cards.push(Card::new(String::from("10"),String::from("♢")));
    game.player.cards.push(Card::new(String::from("A"),String::from("♢")));
    game.player.cards.push(Card::new(String::from("10"),String::from("♢")));

    game.execute_punishment(3, 0, 5, &mut false);
    game.execute_punishment(3, 3, 5, &mut false);

    let comp_right = game.computers[0].cards.len() - 1;
    let player_right = game.player.cards.len() - 1;

    assert_eq!(game.computers[0].cards[comp_right].display(), "7♣");
    assert_eq!(game.player.cards[player_right].display(), "7♣");
}

#[test]
fn test_points_to_zero(){
    let mut game = GameMatch::new();

    game.computers[0].points = 15;
    game.player.points = 15;

    game.execute_punishment(6, 0, 5, &mut false);
    game.execute_punishment(6, 3, 5, &mut false);

    assert_eq!(game.computers[0].points, 0);
    assert_eq!(game.player.points, 0);
}