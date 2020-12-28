// Advent of Code 2020: December, 10
// Day 10: Adapter Array

use crate::manage_input::parse_numbers_u64;

pub fn answers_day10() -> (u16, u64) {
    let filepath = "inputs/day10_input.txt";
    let input = parse_numbers_u64(filepath);
    let outlet = 0;
    let prepared_data = prepare_data(input, outlet);
    
    // Part 1
    let joltage_diffs = joltage_distribution(&prepared_data);
    let jolt_diff_one_three = joltage_diffs.0 * joltage_diffs.1;

    // Part 2
    let n_arrangements = different_arrangements(&prepared_data);
    
    (jolt_diff_one_three, n_arrangements)
}

fn prepare_data(mut input: Vec<u64>, outlet: u64) -> Vec<u64> {
    &input.push(outlet);
    &input.sort();

    input
}

fn joltage_distribution(input: &Vec<u64>) -> (u16, u16) {
    // Don't need a counter for two jolts.
    let mut one_jolt_diff: u16 = 0;
    // "Finally, your device's built-in adapter is always 3 higher than the highest adapter",
    // so three jolt counter starts at 1.
    let mut three_jolts_diff: u16 = 1;
    
    for i in 0..input.len() - 1 {
        let next = i + 1;

        if input[i] + 1 == input[next] {
            one_jolt_diff += 1;
        }
        else if input[i] + 3 == input[next] {
            three_jolts_diff += 1;
        }
        else {
            break;
        }
    }

    (one_jolt_diff, three_jolts_diff)
}

fn different_arrangements(input: &Vec<u64>) -> u64 {
    // Not my idea. Surprised that the "Tribonacci sequence" exists.
    input.windows(2)
    .collect::<Vec<_>>()
    .split(|n| n[1] - n[0] == 3)
    .map(|n| match n.len() {
        4 => 7,
        3 => 4,
        2 => 2,
        _ => 1
    }).product()
}
