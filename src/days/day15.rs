// Advent of Code 2022
// Day 15: Beacon Exclusion Zone

use std::io::BufRead;
use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day15_input.txt";

pub fn print_answers() {
    let mut day = Day::new(15, FILE.to_string());

    let (positions_sensors, positions_beacons) = parse_input(&day);

    day.first_answer = Some(Answer::Num(0));
    day.second_answer = Some(Answer::Num(0));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> (Vec<(isize, isize)>, Vec<(isize, isize)>) {
    let mut positions_sensors: Vec<(isize, isize)> = Vec::new();
    let mut positions_beacons: Vec<(isize, isize)> = Vec::new();

    for line in day.read_file().lines() {
        let content = line.unwrap()
            .replace("Sensor at x=", "")
            .replace(" y=", "")
            .replace(":", ",")
            .replace(" closest beacon is at x=", "");
        let mut nums = content.split(",")
            .map(|n| n.parse::<isize>()
            .unwrap());
        let sensor = (nums.next().unwrap(), nums.next().unwrap());
        let beacon = (nums.next().unwrap(), nums.next().unwrap());

        positions_sensors.push(sensor);
        positions_beacons.push(beacon);
    }

    (positions_sensors, positions_beacons)
}

// Part 1
fn count_positions_no_beacon_by_row(positions_sensors: &Vec<(isize, isize)>, positions_beacons: &Vec<(isize, isize)>, y: usize) -> u64 {
    todo!()
}

// Part 2
fn calc_second_answer(positions_sensors: &Vec<(isize, isize)>, positions_beacons: &Vec<(isize, isize)>) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day15_input_test.txt";

    #[test]
    fn day15_part1_test() {
        let mut day = Day::new(15, FILE.to_string());
        let (positions_sensors, positions_beacons) = parse_input(&day);

        assert_eq!(26, 26);

        day.first_answer = Some(Answer::Num(0));
    }

    #[test]
    fn day15_part2_test() {
        let mut day = Day::new(15, FILE.to_string());
        

        assert_eq!(5, 5);

        day.second_answer = Some(Answer::Num(0));
    }
}
