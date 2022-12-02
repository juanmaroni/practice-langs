// Advent of Code 2022
// Day 2: Rock Paper Scissors

use std::io::BufRead;

use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day02_input.txt";

pub fn print_answers() {
    let mut day = Day::new(2, FILE.to_string());

    let strat_book = parse_input(&day);

    day.first_answer = Some(Answer::Num(0));
    day.second_answer = Some(Answer::Num(0));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> Vec<(char, char)> {
    let mut strat_book: Vec<(char, char)> = Vec::new();

    for line in day.read_file().lines() {
        let content = line.unwrap();
        let split_chars = content.chars()
                                            .filter(|c| *c != ' ')
                                            .collect::<Vec<char>>();
        strat_book.push((split_chars[0], split_chars[1]));
    }

    strat_book
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

    const FILE: &str = "inputs/tests/day02_input_test.txt";

    #[test]
    fn day02_part1_test() { // Change Day01
        let mut day = Day::new(2, FILE.to_string());
        let strat_book = parse_input(&day);

        assert_eq!(15, 15);

        day.first_answer = Some(Answer::Num(0));
    }

    #[test]
    fn day02_part2_test() { // Change Day01
        let mut day = Day::new(2, FILE.to_string());
        

        assert_eq!(5, 5);

        day.second_answer = Some(Answer::Num(0));
    }
}
