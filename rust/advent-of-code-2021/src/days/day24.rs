// Advent of Code 2021
// Day 24: Arithmetic Logic Unit

use crate::input_handler::parse_program;
use std::collections::HashMap;

const FILE: &str = "inputs/day24_input.txt";

pub fn day24_answer() {
    let program = parse_program(FILE);

    println!("Day 24, part 1: {}", check_valid(program));

    println!("Day 24, part 2: {}\n", 0);
}

fn num_to_vec(num: u64) -> Vec<u8> {
    num.to_string()
       .chars()
       .map(|c| c.to_digit(10).unwrap() as u8)
       .collect()
}

fn execute(instructions: &Vec<Vec<String>>, input: &Vec<u8>) -> HashMap<String, i64> {
    let mut iter_input = input.iter();
    let mut variables: HashMap<String, i64> = HashMap::from([
        (String::from("w"), 0),
        (String::from("x"), 0),
        (String::from("y"), 0),
        (String::from("z"), 0),
    ]);

    for instr in instructions {
        let op = &instr[0];
        let var = &instr[1];

        if op == "inp" {
            *variables.get_mut(var).unwrap() = *iter_input.next().unwrap() as i64;
        }
        else {
            let var2 = &instr[2];
            let var2_value = var2.parse::<i64>();
            let value2 = if var2_value.is_ok() {
                                var2_value.unwrap()
                            } else {
                                *variables.get(var2).unwrap()
                            };

            if op == "add" {
                *variables.get_mut(var).unwrap() += value2;
            }
            else if op == "mul" {
                *variables.get_mut(var).unwrap() *= value2;
            }
            else if op == "div" {
                *variables.get_mut(var).unwrap() /= value2;
            }
            else if op == "mod" {
                *variables.get_mut(var).unwrap() %= value2;
            }
            else if op == "eql" {
                *variables.get_mut(var).unwrap() = if *variables.get(var).unwrap() == value2 { 1 } else { 0 };
            }
        }
    }

    variables
}

fn check_valid(instructions: Vec<Vec<String>>) -> u64 {
    let highest: u64 = 99_999_999_999_999;
    let lowest: u64 = 11_111_111_111_111;

    for i in (lowest..highest).rev() {
        let num_vec = num_to_vec(i);

        if !num_vec.contains(&0) {
            let variables = execute(&instructions, &num_vec);

            if *variables.get("z").unwrap() == 0 {
                return i;
            }
        }
    }

    0
}
    

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day24_input_test.txt";

    #[test]
    fn day24_part1_test() {
        let program = parse_program(FILE);
        
        assert_eq!(*execute(&program, &vec!(1, 2)).get("x").unwrap(), -1);
    }

    #[test]
    fn day24_part2_test() {
    
    }
}
