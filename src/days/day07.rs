// Advent of Code 2021
// Day 7: The Treachery of Whales

use crate::input_handler::parse_positions;

const FILE: &str = "inputs/day07_input.txt";

pub fn day07_answer() {
    let mut positions = parse_positions(FILE);

    println!("Day 7, part 1: {}", calculate_fuel_constant(&mut positions));
    println!("Day 7, part 2: {}\n", calculate_fuel_increasing(&mut positions));
}

fn median(v: &mut Vec<u16>) -> u16 {
    v.sort();
    let half = v.len() / 2;

    if v.len() % 2 == 1 {
        v[half]
    }
    else {
        (v[half - 1] + v[half]) / 2
    }
}

fn calculate_fuel_constant(positions: &mut Vec<u16>) -> usize {
    let median = median(positions);

    positions.into_iter().map(|p| (*p as isize - median as isize).abs() as usize).sum()
}

fn mean(v: &mut Vec<u16>) -> isize {
    v.iter().map(|n| *n as isize).sum::<isize>() / v.len() as isize
}

fn fuel_cost(pos: &[u16], target: isize) -> usize {
    pos.iter().map(|&p| {
            let n = (p as isize - target).abs();
            (n * (n + 1) / 2) as usize  // Gauss
        }).sum()
}

fn calculate_fuel_increasing(positions: &mut Vec<u16>) -> usize {
    let mean = mean(positions);

    (-1..=1).map(|d| (fuel_cost(&positions, mean + d))).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day07_input_test.txt";

    #[test]
    fn day07_part1_test() {
        let mut positions = parse_positions(FILE);
        
        assert_eq!(calculate_fuel_constant(&mut positions), 37);
    }

    #[test]
    fn day07_part2_test() {
        let mut positions = parse_positions(FILE);

        assert_eq!(calculate_fuel_increasing(&mut positions), 168);
    }
}
