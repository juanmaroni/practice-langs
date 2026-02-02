// Advent of Code 2021
// Day 21: Dirac Dice

use crate::input_handler::parse_starting_positions;

const FILE: &str = "inputs/day21_input.txt";

pub fn day21_answer() {
    let starting_positions = parse_starting_positions(FILE);
    let (score, turns) = practice_game(&starting_positions);
    
    println!("Day 21, part 1: {}", score as usize * turns as usize);

    println!("Day 21, part 2: {}\n", 0);
}

fn practice_game(starting_positions: &Vec<u16>) -> (u16, u16) {
    let mut pos_player1 = starting_positions[0];
    let mut pos_player2 = starting_positions[1];
    let mut score_player1 = 0;
    let mut score_player2 = 0;
    let mut die = 1;
    let mut times_die_rolled = 0;

    loop {
        // Player 1 plays
        let mut score = (pos_player1 + die % 100 + (die + 1) % 100 + (die + 2) % 100) % 10;

        if score == 0 {
            score = 10;
        }

        pos_player1 = score;
        score_player1 += pos_player1;
        times_die_rolled += 3;

        if score_player1 >= 1000 {
            return (score_player2, times_die_rolled);
        }

        // Player 2 plays
        score = (pos_player2 + (die + 3) % 100 + (die + 4) % 100 + (die + 5) % 100) % 10;

        if score == 0 {
            score = 10;
        }
        
        pos_player2 = score;
        score_player2 += pos_player2;
        times_die_rolled += 3;
        
        if score_player2 >= 1000 {
            return (score_player1, times_die_rolled);
        }

        // Next die number
        die += 6;
    }
}

fn real_game(starting_positions: &Vec<u16>) {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day21_input_test.txt";

    #[test]
    fn day21_part1_test() {
        let starting_positions = parse_starting_positions(FILE);
        let (score, turns) = practice_game(&starting_positions);
        //println!("{:?}", starting_positions);
        assert_eq!(score as usize * turns as usize, 739785);
    }

    #[test]
    fn day21_part2_test() {
        let starting_positions = parse_starting_positions(FILE);
        real_game(&starting_positions);
        //assert_eq!(, 444356092776315);
    }
}
