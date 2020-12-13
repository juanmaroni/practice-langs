// Advent of Code 2020: December, 13
// Day 13: Shuttle Search

use crate::manage_input::get_bus_schedule;
use crate::manage_input::get_bus_ids;

pub fn answers_day13() -> (u32, u64) {
    let filepath: &str = "inputs/day13_input.txt";
    let ans_p1 = answer_part1(earliest_bus(get_bus_schedule(filepath)));
    //println!("{:?}", get_bus_ids(filepath));
    (ans_p1, 0)
}

fn answer_part1(earliest_bus: (u32, u16)) -> u32 {
    earliest_bus.0 * (earliest_bus.1 as u32)
}

fn earliest_bus(schedule: (u32, Vec<u16>)) -> (u32, u16) {
    let mut timestamp = schedule.0;
    let mut found = false;
    let mut bus: u16 = 0;

    while !found {
        for b in &schedule.1 {
            if timestamp % *b as u32 == 0 {
                found = true;
                bus = *b;
                break;
            }
        }

        if !found {
            timestamp += 1;
        }
    }

    (timestamp - schedule.0, bus)
}
