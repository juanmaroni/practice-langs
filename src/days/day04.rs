// Advent of Code 2022
// Day 4: Camp Cleanup

use std::io::BufRead;
use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day04_input.txt";

pub fn print_answers() {
    let mut day = Day::new(4, FILE.to_string());

    let section_assignment_pairs = parse_input(&day);

    day.first_answer = Some(Answer::Num(count_fully_overlapped_pairs(&section_assignment_pairs) as u64));
    day.second_answer = Some(Answer::Num(count_partially_overlapped_pairs(&section_assignment_pairs) as u64));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> Vec<Vec<Vec<u8>>> {
    day.read_file()
        .lines()
        .map(|line| line.unwrap())
        .map(|content| content.split(',')
            .map(|pair| pair.split('-')
                .map(|bound| bound.parse::<u8>().unwrap())
                .collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>())
        .collect()
}

// Part 1
fn count_fully_overlapped_pairs(section_assignment_pairs: &Vec<Vec<Vec<u8>>>) -> u32 {
    section_assignment_pairs.iter()
        .filter(|sections| is_fully_contained(&sections[0], &sections[1]))
        .count() as u32
}

// Part 2
fn count_partially_overlapped_pairs(section_assignment_pairs: &Vec<Vec<Vec<u8>>>) -> u32 {
    section_assignment_pairs.iter()
        .filter(|sections| is_partially_contained(&sections[0], &sections[1]))
        .count() as u32
}

// Helping function for Part 1
fn is_fully_contained(section1: &Vec<u8>, section2: &Vec<u8>) -> bool {
    (section1[0] >= section2[0] && section1[1] <= section2[1]) ||
    (section1[0] <= section2[0] && section1[1] >= section2[1])
}

// Helping function for Part 2
fn is_partially_contained(section1: &Vec<u8>, section2: &Vec<u8>) -> bool {
    !(section1[1] < section2[0] || section2[1] < section1[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day04_input_test.txt";

    #[test]
    fn day04_part4_test() {
        let mut day = Day::new(4, FILE.to_string());
        let section_assignment_pairs = parse_input(&day);
        let ans = count_fully_overlapped_pairs(&section_assignment_pairs);

        assert_eq!(ans, 2);

        day.first_answer = Some(Answer::Num(ans as u64));
    }

    #[test]
    fn day04_part2_test() {
        let mut day = Day::new(4, FILE.to_string());
        let section_assignment_pairs = parse_input(&day);
        let ans = count_partially_overlapped_pairs(&section_assignment_pairs);

        assert_eq!(ans, 4);

        day.second_answer = Some(Answer::Num(ans as u64));
    }
}
