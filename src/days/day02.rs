// Advent of Code 2021
// Day 2: Dive!

use crate::input_handler::parse_commands;

const FILE: &str = "inputs/day02_input.txt";

pub fn day02_answer() {
    let commands = parse_commands(FILE);

    let coordinates = calculate_coordinates(commands.clone());
    println!("Day 2, part 1: {}", coordinates.0 * coordinates.1);

    let coordinates = recalculate_coordinates(commands);
    println!("Day 2, part 2: {}\n", coordinates.0 * coordinates.1);
}

fn calculate_coordinates(commands: Vec<(String, u32)>) -> (u32, u32) {
    let mut depth = 0;
    let mut horizontal = 0;

    for (instr, value) in commands {
        match instr.as_str() {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => ()
        }
    }

    (depth, horizontal)
}

fn recalculate_coordinates(commands: Vec<(String, u32)>) -> (u32, u32) {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for (instr, value) in commands {
        match instr.as_str() {
            "forward" => {
                horizontal += value;
                depth += value * aim
            },
            "down" => aim += value,
            "up" => aim -= value,
            _ => ()
        }
    }

    (depth, horizontal)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day02_input_test.txt";

    #[test]
    fn day02_part1_test() {
        let commands = parse_commands(FILE);
        let coordinates = calculate_coordinates(commands);

        assert_eq!(coordinates.0 * coordinates.1, 150);
    }

    #[test]
    fn day02_part2_test() {
        let commands = parse_commands(FILE);
        let coordinates = recalculate_coordinates(commands);

        assert_eq!(coordinates.0 * coordinates.1, 900);
    }
}
