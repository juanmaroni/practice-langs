// Advent of Code 2021
// Day 4: Giant Squid

use crate::input_handler::parse_bingo;

const FILE: &str = "inputs/day04_input.txt";

pub fn day04_answer() {
    let (draw_numbers, all_possible_lines) = parse_bingo(FILE);
    let score_winner = get_winner_data(draw_numbers.clone(), all_possible_lines.clone());

    println!("Day 4, part 1: {}", score_winner);


    //let score_loser = get_loser_data(draw_numbers, all_possible_lines);

    //println!("Day 4, part 2: {}", score_loser);
}

fn get_winner_data(draw_numbers: Vec<u8>, all_possible_lines: Vec<Vec<u8>>) -> u16 {
    let mut possible_lines = all_possible_lines.clone();

    for n in draw_numbers {
        let mut next_turn: Vec<Vec<u8>> = Vec::new();

        for (i, mut l) in possible_lines.clone().into_iter().enumerate() {
            l.retain(|m| *m != n);

            if l.len() == 0 {
                let index_board: Vec<usize> = (0..5).map(|n| n + (i / 10) * 10).collect();
                let mut sum_nums_left: u16 = 0;

                for idx in index_board {
                    let sum_row: u16 = possible_lines[idx].clone().iter().map(|v| *v as u16).sum();
                    sum_nums_left += sum_row;
                }

                let n = n as u16;
                
                return (sum_nums_left - n) * n;
            }

            next_turn.push(l);
        }
        
        possible_lines = next_turn;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day04_input_test.txt";

    #[test]
    fn day04_part1_test() {
        let (draw_numbers, all_possible_lines) = parse_bingo(FILE);
        let score = get_winner_data(draw_numbers, all_possible_lines);

        assert_eq!(score, 4512);
    }

    #[test]
    fn day04_part2_test() {
        //let (draw_numbers, all_possible_lines) = parse_bingo(FILE);

        //assert_eq!(oxigen_generator_rating * co2_scrubber_rating, 4512);
    }
}
