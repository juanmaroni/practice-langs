// Advent of Code 2021
// Day 4: Giant Squid

use crate::input_handler::parse_bingo;

const FILE: &str = "inputs/day04_input.txt";

pub fn day04_answer() {
    let (draw_numbers, mut boards) = parse_bingo(FILE);        
    let winner_score = get_winner_data(&draw_numbers, &mut boards);

    println!("Day 4, part 1: {}", winner_score);

    //let loser_score = get_loser_data(draw_numbers, all_possible_lines);

    println!("Day 4, part 2: {}\n", 0);
}

fn get_winner_data(draw_numbers: &Vec<u8>, boards: &mut Vec<Vec<Vec<u8>>>) -> u16 {
    for dn in draw_numbers {
        for b in boards.iter_mut() {
            for bl in b.iter_mut() {
                bl.retain(|m| m != dn);

                if bl.len() == 0 {
                    let sum: u16 = b.iter().flat_map(|n| n.iter().map(|n| *n as u16)).sum();
                    return sum * *dn as u16;
                }
            }
        }
    }

    0
}

fn get_loser_data(draw_numbers: &Vec<u8>, boards: &mut Vec<Vec<Vec<u8>>>) -> u16 {
    
    
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day04_input_test.txt";

    #[test]
    fn day04_part1_test() {
        let (draw_numbers, mut boards) = parse_bingo(FILE);        

        assert_eq!(get_winner_data(&draw_numbers, &mut boards), 4512);
    }

    #[test]
    fn day04_part2_test() {
        let (draw_numbers, mut boards) = parse_bingo(FILE);
        get_winner_data(&draw_numbers, &mut boards);
        get_loser_data(&draw_numbers, &mut boards);

        //assert_eq!(get_loser_data(&draw_numbers, &mut boards), 4512);
    }
}
