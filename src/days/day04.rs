// Advent of Code 2021
// Day 4: Giant Squid

use crate::input_handler::{parse_bingo, parse_bingo_boards, transpose_matrix_nums_u8};

const FILE: &str = "inputs/day04_input.txt";

pub fn day04_answer() {
    let (draw_numbers, boards) = parse_bingo(FILE);
    //let score_winner = get_winner_data(draw_numbers.clone(), boards.clone());

    //println!("Day 4, part 1: {}", score_winner);


    //let score_loser = get_loser_data(draw_numbers, all_possible_lines);

    println!("Day 4, part 2: {}\n", 0);
}

fn mark_number(draw_number: u8, mut board: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut board_after: Vec<Vec<u8>> = Vec::new();

    for mut line in board {
        line.retain(|m| *m != draw_number);
        board_after.push(line);
    }

    board_after
}

fn get_winner_data(draw_numbers: Vec<u8>, mut boards: Vec<Vec<Vec<u8>>>) {
    for dn in draw_numbers {
        for b in boards {
            for mut bl in b {
                bl.retain(|m| *m != dn);
            }

            break;
        }
        //println!("{:?}", boards);

        break;
    }

}
/*fn get_winner_data(draw_numbers: Vec<u8>, all_possible_lines: Vec<Vec<u8>>) -> u16 {
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
}*/

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day04_input_test.txt";

    #[test]
    fn day04_part1_test() {
        let (draw_numbers, boards) = parse_bingo_boards(FILE);
        println!("{:?}", boards);
        let score = get_winner_data(draw_numbers, boards);
        
        println!("{:?}", score);

        //assert_eq!(score, 4512);
    }

    #[test]
    fn day04_part2_test() {
        let (draw_numbers, boards) = parse_bingo_boards(FILE);
        println!("{:?}", draw_numbers);
        println!("{:?}", boards);

        //assert_eq!(oxigen_generator_rating * co2_scrubber_rating, 4512);
    }
}
