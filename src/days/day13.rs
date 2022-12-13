// Advent of Code 2022
// Day 13: Distress Signal

use std::io::BufRead;
use itertools::Itertools;
use serde_json;
use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day13_input.txt";

type JSON = serde_json::Value;

pub fn print_answers() {
    let mut day = Day::new(13, FILE.to_string());

    let packets = parse_input(&day);

    day.first_answer = Some(Answer::Num(0));
    day.second_answer = Some(Answer::Num(0));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> Vec<Vec<JSON>> {
    day.read_file()
        .lines()
        .chunks(3)
        .into_iter()
        .map(|ch| ch.take(2)
            .map(|packet| serde_json::from_str(&packet.unwrap())
                .expect("Bad format!"))
            .collect::<Vec<JSON>>())
        .collect()
}

// Part 1
fn count_packets_ordered(packets: &Vec<Vec<JSON>>) -> u64 {
    todo!()
}

// Part 2
fn calc_second_answer(packets: &Vec<Vec<JSON>>) -> u64 {
    todo!()
}

// Helping function for Part 1
fn compare_packets(pair: &Vec<JSON>) {
    let p1 = &pair[0];
    let p2 = &pair[1];

    

}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day13_input_test.txt";

    #[test]
    fn day13_part1_test() {
        let mut day = Day::new(13, FILE.to_string());
        let packets = parse_input(&day);
        //println!("{:?}", );
        compare_packets(&packets[0]);
        assert_eq!(7, 7);

        day.first_answer = Some(Answer::Num(0));
    }

    #[test]
    fn day13_part2_test() {
        let mut day = Day::new(13, FILE.to_string());
        let packets = parse_input(&day);

        assert_eq!(5, 5);

        day.second_answer = Some(Answer::Num(0));
    }
}
