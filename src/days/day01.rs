// Advent of Code 2021: December, 1
// Day 1: Sonar Sweep

use crate::input_handler;

const FILE: &str = "inputs/day01_input.txt";

pub fn day01_answer() {
    let sonar_values = input_handler::read_file_lines_as_nums(FILE);

    println!("Day 1, part 1: {}", count_depth_measurement_increases(sonar_values.clone()));
    println!("Day 1, part 2: {}", count_depth_measurement_increases_grouped_by_3(sonar_values));
}

fn count_depth_measurement_increases(values: Vec<u32>) -> u32 {
    let mut count = 0;
    let mut current_value = values[0];

    for v in &values[1..] {
        if *v > current_value {
            count += 1;
        }

        current_value = *v;
    }

    count
}

fn count_depth_measurement_increases_grouped_by_3(values: Vec<u32>) -> u32 {
    let mut count = 0;
    let mut prev: u32 = values[..3].iter().sum();

    for i in 0..values.len() - 2 {
        let curr= values[i..i + 3].iter().sum();

        if curr > prev {
            count += 1;
        }

        prev = curr;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day01_input_test.txt";

    #[test]
    fn day01_part1_test() {
        let sonar_values = input_handler::read_file_lines_as_nums(FILE);

        assert_eq!(count_depth_measurement_increases(sonar_values), 7);
    }

    #[test]
    fn day01_part2_test() {
        let sonar_values = input_handler::read_file_lines_as_nums(FILE);

        assert_eq!(count_depth_measurement_increases_grouped_by_3(sonar_values), 5);
    }
}
