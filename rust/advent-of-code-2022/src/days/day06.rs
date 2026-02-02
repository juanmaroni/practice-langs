// Advent of Code 2022
// Day 6: Tuning Trouble

use std::io::BufRead;
use itertools::Itertools;
use aoc2022::{Answer, Day, Part};

const FILE: &str = "inputs/real/day06_input.txt";

pub fn print_answers() {
    let mut day = Day::new(6, FILE.to_string());

    let datastream = parse_input(&day);

    day.first_answer = Some(Answer::Num(first_marker_appearance(&datastream)));
    day.second_answer = Some(Answer::Num(first_message_appearance(&datastream)));

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
fn first_marker_appearance(datastream: &String) -> u64 {
    find_text(&datastream, 4) as u64
}

// Part 2
fn first_message_appearance(datastream: &String) -> u64 {
    find_text(&datastream, 14) as u64
}

// Helping function to find first appearance, based on a size
fn find_text(datastream: &String, packet_size: usize) -> usize {
    let mut count = 0;

    for w in datastream.chars().collect::<Vec<char>>().windows(packet_size) {        
        if w.iter().unique().count() == packet_size {
            count += packet_size;
            break;
        }

        count += 1;
    }

    count
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

        let datastream = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        
        assert_eq!(first_marker_appearance(&datastream), 5);

        day.first_answer = Some(Answer::Num(ans));
    }

    #[test]
    fn day06_part2_test() {
        let mut day = Day::new(6, FILE.to_string());
        let datastream = parse_input(&day);
        let ans = first_message_appearance(&datastream);
        
        assert_eq!(ans, 19);

        let datastream = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        
        assert_eq!(first_message_appearance(&datastream), 23);

        day.second_answer = Some(Answer::Num(ans));
    }
}
