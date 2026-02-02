// Advent of Code 2022
// Day 17: Pyroclastic Flow

use std::io::BufRead;

use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day17_input.txt";
const CHAMBER_WIDTH: u8 = 7;

pub fn print_answers() {
    let mut day = Day::new(17, FILE.to_string());

    let jet_patterns = parse_input(&day);

    day.first_answer = Some(Answer::Num(0));
    day.second_answer = Some(Answer::Num(0));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> Vec<char> {
    day.read_file()
        .lines()
        .nth(0)
        .unwrap()
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
}

// Part 1
fn calc_first_answer(jet_patterns: &Vec<char>) -> u64 {
    todo!()
}

// Part 2
fn calc_second_answer(jet_patterns: &Vec<char>) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day17_input_test.txt";

    #[test]
    fn day17_part1_test() {
        let mut day = Day::new(17, FILE.to_string());
        

        assert_eq!(7, 7);

        day.first_answer = Some(Answer::Num(0));
    }

    #[test]
    fn day17_part2_test() {
        let mut day = Day::new(17, FILE.to_string());
        

        assert_eq!(5, 5);

        day.second_answer = Some(Answer::Num(0));
    }
}
