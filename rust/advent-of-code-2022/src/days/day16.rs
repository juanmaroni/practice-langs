// Advent of Code 2022
// Day 16: Proboscidea Volcanium

use std::io::BufRead;

use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day16_input.txt";

#[derive(Debug)]
struct Valve {
    id: String,
    flow_rate: u8,
    connections: Vec<String>,
}
pub fn print_answers() {
    let mut day = Day::new(16, FILE.to_string());

    let valves = parse_input(&day);

    day.first_answer = Some(Answer::Num(0));
    day.second_answer = Some(Answer::Num(0));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> Vec<Valve> {
    let mut valves: Vec<Valve> = Vec::new();

    for line in day.read_file().lines() {
        let content = line.unwrap()
            .replace("Valve ", "")
            .replace("has flow rate=", "")
            .replace("; tunnels lead to valves", "")
            .replace("; tunnel leads to valve", "")
            .replace(", ", ",");

        let mut split = content.split_whitespace();
        
        valves.push(Valve {
            id: String::from(split.next().unwrap()),
            flow_rate: split.next().unwrap().parse::<u8>().unwrap(),
            connections: split.next().unwrap().split(",").map(|s| String::from(s)).collect(),
        });
    }

    valves
}

// Part 1
fn calc_first_answer(valves: &Vec<Valve>) -> u64 {
    todo!()
}

// Part 2
fn calc_second_answer(valves: &Vec<Valve>) -> u64 {
    todo!()
}

// Helping function for Part 1
fn navigate_tunnels(valves: &Vec<Valve>) -> u64 {
    let cycle = 30;

    while cycle > 0 {
        
    }

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day16_input_test.txt";

    #[test]
    fn day16_part1_test() {
        let mut day = Day::new(16, FILE.to_string());
        let valves = parse_input(&day);
        println!("{:?}", valves);

        assert_eq!(1651, 1651);

        day.first_answer = Some(Answer::Num(0));
    }

    #[test]
    fn day16_part2_test() {
        let mut day = Day::new(16, FILE.to_string());
        

        assert_eq!(5, 5);

        day.second_answer = Some(Answer::Num(0));
    }
}
