// Advent of Code 2022
// Day 15: Beacon Exclusion Zone

use std::{io::BufRead, collections::HashMap};
use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day15_input.txt";

pub fn print_answers() {
    let mut day = Day::new(15, FILE.to_string());

    //let positions_sensors_beacons = parse_input(&day);

    day.first_answer = Some(Answer::Num(0));
    day.second_answer = Some(Answer::Num(0));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> HashMap<(isize, isize), Vec<(isize, isize)>> {
    let mut beacon_sensors: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
    let mut positions_sensors_beacons: Vec<((isize, isize), (isize, isize))> = Vec::new();

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

        create_or_add_to_hashmap(beacon, sensor, &mut beacon_sensors);
    }

    beacon_sensors
}

// Helping function for parsing
fn create_or_add_to_hashmap(k: (isize, isize), v: (isize, isize), hm: &mut HashMap<(isize, isize), Vec<(isize, isize)>>) {
    if !hm.contains_key(&k) {
        hm.insert(k, vec![v]);
    } else {
        hm.get_mut(&k).unwrap().push(v);
    }
}

// Part 1
fn count_positions_no_beacon_by_row(beacon_sensors: &HashMap<(isize, isize), Vec<(isize, isize)>>, row: isize) -> u64 { //5350384
    // Calculate distance from given row to sensors
    // Different points on that row
    let closest_sensors = beacon_sensors.keys()
        .filter(|(_, y)| *y == row)
        .flat_map(|k| beacon_sensors.get(k)
            .unwrap()
            .clone())
        .collect::<Vec<(isize, isize)>>();
    println!("{:?}", closest_sensors);
    
    
    0
}

// Part 2
fn calc_second_answer(beacon_sensors: &HashMap<(isize, isize), Vec<(isize, isize)>>) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day15_input_test.txt";

    #[test]
    fn day15_part1_test() {
        let mut day = Day::new(15, FILE.to_string());
        let beacon_sensors = parse_input(&day);
        //println!("{:?}", beacon_sensors);
        count_positions_no_beacon_by_row(&beacon_sensors, 10);
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
