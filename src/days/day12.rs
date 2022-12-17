// Advent of Code 2022
// Day 12: Hill Climbing Algorithm

use std::io::BufRead;

use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day12_input.txt";

struct Node {
    height: char,
    next: Vec<Node>,
}

pub fn print_answers() {
    let mut day = Day::new(12, FILE.to_string());

    let heightmap = parse_input(&day);

    day.first_answer = Some(Answer::Num(0));
    day.second_answer = Some(Answer::Num(0));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> Vec<Vec<char>> {
    day.read_file()
        .lines()
        .map(|line| line.unwrap()
            .chars()
            .collect::<Vec<char>>())
        .collect()
}

// Part 1
fn count_steps_shortest_path(heightmap: &Vec<Vec<char>>) -> u32 {
    todo!()
}

// Part 2
fn calc_second_answer(heightmap: &Vec<Vec<char>>) -> u32 {
    todo!()
}

// Helping function for Part 1
fn build_tree(heightmap: &Vec<Vec<char>>) {

}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day12_input_test.txt";

    #[test]
    fn day12_part1_test() {
        let mut day = Day::new(12, FILE.to_string());
        

        assert_eq!(7, 7);

        day.first_answer = Some(Answer::Num(0));
    }

    #[test]
    fn day12_part2_test() {
        let mut day = Day::new(12, FILE.to_string());
        

        assert_eq!(5, 5);

        day.second_answer = Some(Answer::Num(0));
    }
}
