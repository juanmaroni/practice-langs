// Advent of Code 2022
// Day 2: Rock Paper Scissors

use std::io::BufRead;
use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day02_input.txt";

pub fn print_answers() {
    let mut day = Day::new(2, FILE.to_string());

    let strat_guide = parse_input(&day);

    day.first_answer = Some(Answer::Num(calc_wrong_score(&strat_guide) as u64));
    day.second_answer = Some(Answer::Num(calc_right_score(&strat_guide) as u64));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> Vec<(char, char)> {
    let mut strat_guide: Vec<(char, char)> = Vec::new();

    for line in day.read_file().lines() {
        let content = line.unwrap();
        let split_chars = content.chars()
                                            .filter(|c| *c != ' ')
                                            .collect::<Vec<char>>();
        strat_guide.push((split_chars[0], split_chars[1]));
    }

    strat_guide
}

// Part 1
fn calc_wrong_score(strat_guide: &Vec<(char, char)>) -> u32 {
    strat_guide.iter().map(|play| match_play_wrong(play)).sum()
}

// Part 2
fn calc_right_score(strat_guide: &Vec<(char, char)>) -> u32 {
    strat_guide.iter().map(|play| match_play_right(play)).sum()
}

// This is so dirty... XD
fn match_play_wrong(play: &(char, char)) -> u32 {
    match play {
        ('A', 'X') => 1 + 3,
        ('B', 'X' ) => 1,
        ('C', 'X') => 1 + 6,
        ('A', 'Y') => 2 + 6,
        ('B', 'Y' ) => 2 + 3,
        ('C', 'Y') => 2,
        ('A', 'Z') => 3,
        ('B', 'Z' ) => 3 + 6,
        ('C', 'Z') => 3 + 3,
        _ => panic!("Unrecognized play!"),
    }
}

fn match_play_right(play: &(char, char)) -> u32 {
    match play {
        ('A', 'Z') => 6 + 2,
        ('B', 'Z' ) => 6 + 3,
        ('C', 'Z') => 6 + 1,
        ('A', 'Y') => 3 + 1,
        ('B', 'Y' ) => 3 + 2,
        ('C', 'Y') => 3 + 3,
        ('A', 'X') => 3,
        ('B', 'X' ) => 1,
        ('C', 'X') => 2,
        _ => panic!("Unrecognized play!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day02_input_test.txt";

    #[test]
    fn day02_part1_test() {
        let mut day = Day::new(2, FILE.to_string());
        let strat_guide = parse_input(&day);
        let ans = calc_wrong_score(&strat_guide);

        assert_eq!(ans, 15);

        day.first_answer = Some(Answer::Num(ans as u64));
    }

    #[test]
    fn day02_part2_test() {
        let mut day = Day::new(2, FILE.to_string());
        let strat_guide = parse_input(&day);
        let ans = calc_right_score(&strat_guide);

        assert_eq!(ans, 12);

        day.second_answer = Some(Answer::Num(ans as u64));
    }
}
