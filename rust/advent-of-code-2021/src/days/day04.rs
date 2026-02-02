// Advent of Code 2021
// Day 4: Giant Squid

use crate::input_handler::parse_bingo;

const FILE: &str = "inputs/day04_input.txt";

pub fn day04_answer() {
    let (draw_numbers, mut boards) = parse_bingo(FILE);
    let (winner_score, loser_score) = get_match_data(&draw_numbers, &mut boards);

    println!("Day 4, part 1: {}", winner_score);

    println!("Day 4, part 2: {}\n", loser_score);
}

fn play(draw_numbers: &Vec<u8>, boards: &mut Vec<Vec<Vec<u8>>>) -> (usize, u32) {
    for dn in draw_numbers {
        for (i, b) in boards.iter_mut().enumerate() {
            for bl in b.iter_mut() {
                bl.retain(|m| m != dn);

                if bl.len() == 0 {
                    let sum: u32 = b.iter().flat_map(|n| n.iter().map(|n| *n as u32)).sum();
                    return (i, sum * *dn as u32);
                }
            }
        }
    }

    (0, 0)
}

fn get_match_data(draw_numbers: &Vec<u8>, boards: &mut Vec<Vec<Vec<u8>>>) -> (u32, u32) {
    // Play the first time to get the winner and remove its boards
    let (winner_idx, winner_score) = play(draw_numbers, boards);
    let winner_idx_2 = {
        if winner_idx % 2 == 0 {
            winner_idx
        }
        else {
            winner_idx - 1
        }
    };
    boards.remove(winner_idx);
    boards.remove(winner_idx_2);

    // Play the other boards until all of them are removed, the last one is the loser
    let mut loser_score = 0;

    while boards.len() > 0 {
        let (current_idx, current_score) = play(draw_numbers, boards);
        let current_idx_2 = {
            if current_idx % 2 == 0 {
                current_idx
            }
            else {
                current_idx - 1
            }
        };
        boards.remove(current_idx);
        boards.remove(current_idx_2);

        if boards.len() == 0 {
            loser_score = current_score;
        }
    }

    (winner_score, loser_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day04_input_test.txt";

    #[test]
    fn day04_part1_test() {
        let (draw_numbers, mut boards) = parse_bingo(FILE);
        let (_, winner_score) = play(&draw_numbers, &mut boards);

        assert_eq!(winner_score, 4512);
    }

    #[test]
    fn day04_part2_test() {
        let (draw_numbers, mut boards) = parse_bingo(FILE);
        let (_, loser_score) = get_match_data(&draw_numbers, &mut boards);

        assert_eq!(loser_score, 1924);
    }
}
