// Advent of Code 2020: December, 10
// Day 10: Adapter Array

use crate::manage_input;

pub fn answers_day10() -> (u16, u64) {
    let filepath: &str = "inputs/day10_input.txt";
    let input = manage_input::get_numbers(filepath);
    let outlet = 0;
    
    // Part 1
    let joltages_diffs = joltage_distribution(prepare_data(input, outlet));
    let jolt_diff_one_three = joltages_diffs.0 * joltages_diffs.2;

    // Part 2
    let input = manage_input::get_numbers(filepath);
    let n_arrangements = different_arrangements(prepare_data(input, outlet));
    
    (jolt_diff_one_three, n_arrangements)
}

fn prepare_data(mut input: Vec<u64>, outlet: u64) -> Vec<u64> {
    &input.push(outlet);
    &input.sort();

    input
}

fn joltage_distribution(input: Vec<u64>) -> (u16, u16, u16) {
    let mut one_jolt_diff: u16 = 0;
    let mut two_jolt_diff: u16 = 0;     // Not necessary.
    let mut three_jolt_diff: u16 = 0;
    let mut i: usize = 0;
    
    while i < input.len() - 1 {
        if input[i] + 1 == input[i + 1] {
            one_jolt_diff += 1;
        }
        else if input[i] + 2 == input[i + 1] {
            two_jolt_diff += 1;
        }
        else if input[i] + 3 == input[i + 1] {
            three_jolt_diff += 1;
        }
        else {
            break;
        }

        i += 1;
    }

    // Finally, your device's built-in adapter is always 3 higher than the highest adapter.
    three_jolt_diff += 1;

    (one_jolt_diff, two_jolt_diff, three_jolt_diff)
}

fn different_arrangements(input: Vec<u64>) -> u64 {
    // This wasn't my idea, but I am glad I stumbled upon something new to learn.
    // As I understood it: 
    input.iter()    // transforms the input into an iterator,
    .zip(input.iter().skip(1))  // create an iterator of pairs from the input, like (pair, pair.next),
    .map(|(a, b)| b - a)    // calculates max difference between the elements from each pair
    // And this was the harder part to understand: passing the resulting iterator to fold.
    // Match the difference between pairs (1, 2 or 3) and update the acumulator (tuple of diffs).
    .fold((1, 0, 0), |(diff1, diff2, diff3), diff| match diff {
        1 => (diff1 + diff2 + diff3, diff1, diff2), // If 1, there could be also a 2 and 3 difference.
        2 => (diff1 + diff2, 0, diff1),             // If 2, there could be also a 3 difference.
        3 => (diff1, 0, 0),                         // If 3, no other diff works.
        _ => unreachable!(),                        // Bigger difference in jolts, we are f*cked.
    }).0    // Extract and return first element from the resulting tuple.

    // Well, hoping that I have understood it.
}
