// Advent of Code 2021
// Day 8: Seven Segment Search

use crate::input_handler::parse_patterns_outputs;

const FILE: &str = "inputs/day08_input.txt";

pub fn day08_answer() {
    let patterns_outputs = parse_patterns_outputs(FILE);

    println!("Day 8, part 1: {}", count_unique(&patterns_outputs));
}

fn count_unique(patterns_outputs: &Vec<([String; 10], [String; 4])>) -> u16 {
    // Unique patterns are the numbers (with their lengths): 1 (6), 4 (4), 7 (3) and 8 (7)
    let mut counter = 0;

    for (_, o) in patterns_outputs {
        counter += o.iter()
                    .map(|o| o.len())
                    .filter(|len| *len == 2 || *len == 4 || *len == 3 || *len == 7)
                    .count() as u16;
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day08_input_test.txt";

    #[test]
    fn day08_part1_test() {
        let patterns_outputs = parse_patterns_outputs(FILE);
        
        assert_eq!(count_unique(&patterns_outputs), 26);
    }

    #[test]
    fn day08_part2_test() {
        let patterns_outputs = parse_patterns_outputs(FILE);

    }
}
