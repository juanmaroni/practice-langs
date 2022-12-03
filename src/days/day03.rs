// Advent of Code 2022
// Day 3: Rucksack Reorganization

use std::io::BufRead;
use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day03_input.txt";

pub fn print_answers() {
    let mut day = Day::new(3, FILE.to_string());
    
    let compartiments = parse_input(&day);

    day.first_answer = Some(Answer::Num(sum_priorities(&compartiments) as u64));
    day.second_answer = Some(Answer::Num(0));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> Vec<String> {
    day.read_file()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>()
}

// Part 1
fn sum_priorities(compartiments: &Vec<String>) -> u32 {
    compartiments.iter()
        .map(|comp| convert_to_priority(find_common_item(comp)))
        .sum()
}

fn calc_second_answer(values: &Vec<String>) -> u32 {
    todo!()
}

fn find_common_item(compartiment: &String) -> char {
    let len_compartiment = compartiment.len() / 2;
    let first_comp = &compartiment[..len_compartiment];
    let second_comp = &compartiment[len_compartiment..];

    for c in first_comp.chars() {
        if second_comp.contains(c) {
            return c;
        }
    }

    // If not found, return any non-alphabetic char
    '$'
}

fn convert_to_priority(item: char) -> u32 {
    if item == '$' {
        0
    } else {
        if item >= 'a' && item <= 'z' {
            (item as u8 - 96) as u32
        } else {
            (item as u8 - 38) as u32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day03_input_test.txt";

    #[test]
    fn day03_part1_test() {
        let mut day = Day::new(1, FILE.to_string());
        let compartiments = parse_input(&day);
        let ans = sum_priorities(&compartiments);

        assert_eq!(ans, 157);

        day.first_answer = Some(Answer::Num(ans as u64));
    }

    #[test]
    fn day03_part2_test() {
        let mut day = Day::new(1, FILE.to_string());
        

        assert_eq!(5, 5);

        day.second_answer = Some(Answer::Num(0));
    }
}
