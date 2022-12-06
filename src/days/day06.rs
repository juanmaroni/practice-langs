// Advent of Code 2022
// Day 6: Tuning Trouble

use std::io::BufRead;
use itertools::Itertools;
use aoc2022::{Answer, Day, Part};

const FILE: &str = "inputs/real/day06_input.txt";

pub fn print_answers() {
    let mut day = Day::new(6, FILE.to_string());

    let datastream = parse_input(&day);

    day.first_answer = Some(Answer::Num(first_marker_appearance(&datastream) as u64));
    day.second_answer = Some(Answer::Num(first_message_appearance(&datastream) as u64));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> String {
    day.read_file()
        .lines()
        .map(|line| line.unwrap())
        .collect()
}

// Part 1
fn first_marker_appearance(datastream: &String) -> u32 {
    find_marker(&datastream).1
}

// Part 2
fn first_message_appearance(datastream: &String) -> u32 {
    find_message(&datastream).1
}

fn find_marker(datastream: &String) -> (String, u32) {
    let mut marker = String::new();
    let mut count = 0;

    for w in datastream.chars().collect::<Vec<char>>().windows(4) {
        marker = w.iter().unique().collect::<String>();
        
        if marker.len() == 4 {
            count += 4;
            break;
        }

        count += 1;
    }

    (marker, count)
}

fn find_message(datastream: &String) -> (String, u32) {
    let mut message = String::new();
    let mut count = 0;

    for w in datastream.chars().collect::<Vec<char>>().windows(14) {
        message = w.iter().unique().collect::<String>();
        
        if message.len() == 14 {
            count += 14;
            break;
        }

        count += 1;
    }

    (message, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day06_input_test.txt";

    #[test]
    fn day06_part1_test() {
        let mut day = Day::new(6, FILE.to_string());
        let datastream = parse_input(&day);
        let ans = first_marker_appearance(&datastream);
        assert_eq!(ans, 7);

        let datastream2 = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(first_marker_appearance(&datastream2), 5);

        day.first_answer = Some(Answer::Num(ans as u64));
    }

    #[test]
    fn day06_part2_test() {
        let mut day = Day::new(6, FILE.to_string());
        let datastream = parse_input(&day);
        let ans = first_message_appearance(&datastream);
        assert_eq!(ans, 19);

        let datastream2 = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(first_message_appearance(&datastream2), 23);

        day.second_answer = Some(Answer::Num(ans as u64));
    }
}
