// Advent of Code 2022
// Day 5: Supply Stacks

use std::io::BufRead;
use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day05_input.txt";

#[derive(Debug)]
struct RearrangementMove {
    quantity: u8,
    from: u8,
    to: u8,
}

pub fn print_answers() {
    let mut day = Day::new(5, FILE.to_string());

    let (mut stacks, rearrangements) = parse_input(&day, 9);
    
    day.first_answer = Some(Answer::Str(get_incorrect_message(&mut stacks.clone(), &rearrangements)));
    day.second_answer = Some(Answer::Str(get_correct_message(&mut stacks, &rearrangements)));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

// See the input to know the size, it's the number of stacks
fn parse_input(day: &Day, size: u8) -> (Vec<Vec<char>>, Vec<RearrangementMove>) {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut rearrangements: Vec<RearrangementMove> = Vec::new();

    for _ in 0..size {
        stacks.push(Vec::new());
    }

    let mut lines = day.read_file().lines();

    // Parse drawing
    for line in &mut lines {
        let content = line.unwrap();

        // No numbers, thanks
        if !content.chars().any(|c| c.is_alphabetic()) {
            break;
        }

        let content = content.replace("    ", ". ")
                    .replace("[", "")
                    .replace("]", "")
                    .replace(" ", "");      // Ashamed of this

        for (i, c) in content.chars().enumerate() {
            if c != '.' {
                stacks[i].insert(0, c);
            }
        }
    }

    // Skip empty line
    lines.next();

    // Parse rearrangement procedure
    for line in lines {
        let content = line.unwrap();
        let split = content.split_whitespace()
                                    .collect::<Vec<&str>>();
        rearrangements.push(RearrangementMove { quantity: split[1].parse::<u8>().unwrap(),
                                                from: split[3].parse::<u8>().unwrap() - 1,
                                                to: split[5].parse::<u8>().unwrap() - 1 });
    }

    (stacks, rearrangements)
}

// Part 1
fn get_incorrect_message(stacks: &mut Vec<Vec<char>>, rearrangements: &Vec<RearrangementMove>) -> String {
    for rp in rearrangements {
        execute_rearrangement(stacks, &rp)
    }

    stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
}

// Part 2
fn get_correct_message(stacks: &mut Vec<Vec<char>>, rearrangements: &Vec<RearrangementMove>) -> String {
    for rp in rearrangements {
        execute_improved_rearrangement(stacks, &rp)
    }

    stacks.iter().map(|stack| stack[stack.len() - 1]).collect()
}

// CrateMover 9000
fn execute_rearrangement(stacks: &mut Vec<Vec<char>>, rp: &RearrangementMove) {
    for _ in 0..rp.quantity {
        let e = stacks[rp.from as usize].pop().unwrap();
        stacks[rp.to as usize].push(e);
    }
}

// CrateMover 9001
fn execute_improved_rearrangement(stacks: &mut Vec<Vec<char>>, rp: &RearrangementMove) {
    let rp_to = rp.to as usize;
    let stack_len = stacks[rp_to].len();

    for _ in 0..rp.quantity {
        let e = stacks[rp.from as usize].pop().unwrap();
        stacks[rp_to].insert(stack_len, e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day05_input_test.txt";

    #[test]
    fn day05_part1_test() {
        let mut day = Day::new(5, FILE.to_string());
        let (mut stacks, rearrangements) = parse_input(&day, 3);
        let ans = get_incorrect_message(&mut stacks, &rearrangements);

        assert_eq!(&ans, "CMZ");

        day.first_answer = Some(Answer::Str(ans));
    }

    #[test]
    fn day05_part2_test() {
        let mut day = Day::new(5, FILE.to_string());
        let (mut stacks, rearrangements) = parse_input(&day, 3);
        let ans = get_correct_message(&mut stacks, &rearrangements);

        assert_eq!(&ans, "MCD");

        day.second_answer = Some(Answer::Str(ans));
    }
}
