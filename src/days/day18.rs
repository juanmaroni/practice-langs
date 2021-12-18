// Advent of Code 2021
// Day 18: Snailfish

use crate::input_handler::parse_snailfish_homework;

const FILE: &str = "inputs/day18_input.txt";

pub fn day18_answer() {
    let snailfish_homework = parse_snailfish_homework(FILE);
    
    println!("Day 18, part 1: {}", 0);

    println!("Day 18, part 2: {}\n", 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day18_input_test.txt";

    #[test]
    fn day18_part1_test() {
        let snailfish_numbers = parse_snailfish_homework(FILE);
        println!("{:?}", snailfish_numbers);
        //assert_eq!(find_velocities(target_area_x, target_area_y).0, 4140);
    }

    #[test]
    fn day18_part2_test() {
        
        //assert_eq!(find_velocities(target_area_x, target_area_y).1, 112);
    }
}
