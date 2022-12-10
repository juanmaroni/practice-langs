// Advent of Code 2022
// Day 10: Cathode-Ray Tube

use std::io::BufRead;
use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day10_input.txt";

pub fn print_answers() {
    let mut day = Day::new(10, FILE.to_string());

    let instructions = parse_input(&day);

    day.first_answer = Some(Answer::Num(find_signal_strength_for_given_cycles(&instructions)));
    day.second_answer = Some(Answer::Num(0));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> Vec<(String, i8)> {
    let mut instructions: Vec<(String, i8)> = Vec::new();
    
    for line in day.read_file().lines() {
        let content = line.unwrap();
        let mut split = content.split_whitespace();
        let inst_name = split.next().unwrap().to_string();
        let inst_value = if inst_name == "noop" {
            0
        } else {
            split.next().unwrap().parse::<i8>().unwrap()
        };

        instructions.push((inst_name, inst_value));
    }

    instructions
}

// Part 1
fn find_signal_strength_for_given_cycles(instructions: &Vec<(String, i8)>) -> u64 {
    //Find the signal strength during the 20th, 60th, 100th, 140th, 180th, and 220th cycles
    execute_instructions_given_cycles(&instructions).iter().sum::<i32>() as u64
}

// Part 2
fn calc_second_answer(instructions: &Vec<(String, i8)>) -> u64 {
    todo!()
}

// Helping functions for Part 1
fn execute_instructions_given_cycles(instructions: &Vec<(String, i8)>) -> Vec<i32>{
    let mut reg_x: i32 = 1;
    let mut cycles: u16 = 0;
    let mut signal_strengths: Vec<i32> = Vec::new();

    for (n, v) in instructions {
        if n == "addx" {
            // Cycle 1/2
            cycles += 1;

            let match_cycles = match_asked_cycles(cycles, reg_x);

            if match_cycles.is_some() {
                signal_strengths.push(match_cycles.unwrap());
            }
            
            // Cycle 2/2
            cycles += 1;
            

            let match_cycles = match_asked_cycles(cycles, reg_x);

            if match_cycles.is_some() {
                signal_strengths.push(match_cycles.unwrap());
            }

            reg_x += *v as i32;
        } else {
            cycles += 1;

            let match_cycles = match_asked_cycles(cycles, reg_x);

            if match_cycles.is_some() {
                signal_strengths.push(match_cycles.unwrap());
            }
        }
    }

    signal_strengths
}

fn match_asked_cycles(cycle: u16, reg_x: i32) -> Option<i32> {
    match cycle {
        20 | 60 | 100 | 140 | 180 | 220 => Some(cycle as i32 * reg_x),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day10_input_test.txt";

    #[test]
    fn day10_part1_test() {
        let mut day = Day::new(10, FILE.to_string());
        let instructions = parse_input(&day);
        let ans = find_signal_strength_for_given_cycles(&instructions);

        assert_eq!(ans, 13140);

        day.first_answer = Some(Answer::Num(ans));
    }

    #[test]
    fn day10_part2_test() {
        let mut day = Day::new(10, FILE.to_string());
        

        assert_eq!(5, 5);

        day.second_answer = Some(Answer::Num(0));
    }
}
