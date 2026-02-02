// Advent of Code 2020: December, 22
// Day 22: Crab Combat

use crate::manage_input::get_decks;

pub fn answers_day22() -> (u16, u64) {
    let filepath: &str = "inputs/day22_input.txt";
    let decks = get_decks(filepath);

    (calculate_score(play_game(decks)), 0)
}

fn play_game(decks: Vec<Vec<u8>>) -> Vec<u8> {
    let mut deck_player1 = decks[0].clone();
    let mut deck_player2 = decks[1].clone();

    while deck_player1.len() > 0 && deck_player2.len() > 0 {
        if deck_player1[0] > deck_player2[0] {
            deck_player1.push(deck_player1[0]);
            deck_player1.push(deck_player2[0]);
        }
        else {
            deck_player2.push(deck_player2[0]);
            deck_player2.push(deck_player1[0]);
        }

        deck_player1.remove(0);
        deck_player2.remove(0);
    }

    if deck_player1.len() > 0 {deck_player1} else {deck_player2}
}

fn calculate_score(deck: Vec<u8>) -> u16 {
    let mut score: u16 = 0;
    let deck_len = deck.len();

    for i in (1..deck_len + 1).rev() {
        score = score + deck[deck_len - i] as u16 * i as u16;
    }

    score
}
