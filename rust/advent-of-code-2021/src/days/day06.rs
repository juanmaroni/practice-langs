// Advent of Code 2021
// Day 6: Lanternfish

use crate::input_handler::parse_line_nums;

const FILE: &str = "inputs/day06_input.txt";

pub fn day06_answer() {
    let fishes = parse_line_nums(FILE);
    let after_80days: usize = simulate_n_days(fishes.clone(), 80).iter().sum();

    println!("Day 6, part 1: {}", after_80days);

    let after_256days: usize = simulate_n_days(fishes, 256).iter().sum();

    println!("Day 6, part 2: {}\n", after_256days);
}

// There are 9 timers, from 0 to 8
// This method updates timers on a day
fn simulate_day(mut fishes: [usize; 9]) -> [usize; 9] {    
    let new_fish = fishes[0];   // Amount of fishes with timer = 0
    
    for timer in 0..8 {
        fishes[timer] = fishes[timer + 1];
    }

    fishes[6] += new_fish;  // A lanternfish that creates a new fish resets its timer to 6
    fishes[8] = new_fish;   // The new lanternfish starts with an internal timer of 8

    fishes
}

fn simulate_n_days(mut fishes: [usize; 9], days: usize) -> [usize; 9] {
    for _ in 0..days {
        fishes = simulate_day(fishes);
    }

    fishes
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day06_input_test.txt";

    #[test]
    fn day06_part1_test() {
        let fishes = parse_line_nums(FILE);
        let after_80days: usize = simulate_n_days(fishes.clone(), 80).iter().sum();

        assert_eq!(after_80days, 5934);
    }

    #[test]
    fn day06_part2_test() {
        let fishes = parse_line_nums(FILE);
        let after_256days: usize = simulate_n_days(fishes, 256).iter().sum();

        assert_eq!(after_256days, 26984457539);
    }
}
