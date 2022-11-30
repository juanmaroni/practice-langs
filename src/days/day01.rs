// Advent of Code 2022
// Day 1: TITLE OF THE DAY

use std::io::BufRead;

use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day01_input.txt";

pub fn print_answers() {
    let mut day = Day::new(1, FILE.to_string());
    day.first_answer = Some(Answer::Num(0));
    day.second_answer = Some(Answer::Num(0));

    // let input = parse_input(day);

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: Day) {
    for line in day.read_file().lines() {
        
    }
}

fn calc_first_answer(values: Vec<u32>) -> u32 {
    todo!()
}

fn calc_second_answer(values: Vec<u32>) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day01_input_test.txt";

    #[test]
    fn day01_part1_test() { // Change Day01
        let mut day = Day::new(1, FILE.to_string());
        day.first_answer = Some(Answer::Num(0));
        day.second_answer = Some(Answer::Num(0));

        assert_eq!(7, 7);
    }

    #[test]
    fn day01_part2_test() { // Change Day01
        let mut day = Day::new(1, FILE.to_string());
        day.first_answer = Some(Answer::Num(0));
        day.second_answer = Some(Answer::Num(0));

        assert_eq!(5, 5);
    }
}
