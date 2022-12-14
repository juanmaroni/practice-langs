// Advent of Code 2022
// Day 14: Regolith Reservoir

use std::io::BufRead;
use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day14_input.txt";

pub fn print_answers() {
    let mut day = Day::new(14, FILE.to_string());

    let rockpaths = parse_input(&day);

    day.first_answer = Some(Answer::Num(0));
    day.second_answer = Some(Answer::Num(0));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> Vec<Vec<Vec<u16>>> {
    day.read_file()
        .lines()
        .map(|line| line.unwrap()
            .split(" -> ")
            .map(|points| points.split(',')
                .map(|p| p.parse::<u16>().unwrap())
                .collect::<Vec<u16>>())
            .collect::<Vec<Vec<u16>>>())
        .collect()
}

// Part 1
fn calc_first_answer(values: Vec<u32>) -> u64 {
    todo!()
}

// Part 2
fn calc_second_answer(values: Vec<u32>) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day14_input_test.txt";

    #[test]
    fn day14_part1_test() {
        let mut day = Day::new(14, FILE.to_string());
        let rockpaths = parse_input(&day);
        println!("{:?}", rockpaths);

        assert_eq!(7, 7);

        day.first_answer = Some(Answer::Num(0));
    }

    #[test]
    fn day14_part2_test() {
        let mut day = Day::new(14, FILE.to_string());
        

        assert_eq!(5, 5);

        day.second_answer = Some(Answer::Num(0));
    }
}
