// Advent of Code 2021
// Day 22: Reactor Reboot

use crate::input_handler::parse_cuboids;
use std::collections::HashSet;

const FILE: &str = "inputs/day22_input.txt";

pub fn day22_answer() {
    let cuboids = parse_cuboids(FILE);

    println!("Day 22, part 1: {}", count_cuboids_on(&cuboids, 50));

    println!("Day 22, part 2: {}\n", 0);
}

fn include(limit: i32, ranges: &Vec<Vec<i32>>) -> bool {
    // 0 for no limit
    limit == 0 || ranges.iter()
        .flat_map(|axis| axis.iter().map(|p| *p <= limit && *p >= -limit))
        .any(|b| b == true)
}

fn count_cuboids_on(cuboids: &Vec<(bool, Vec<Vec<i32>>)>, limit: i32) -> usize {
    let mut unique: HashSet<(i32, i32, i32)> = HashSet::new();

    for (status, ranges) in cuboids {
        if include(limit, &ranges) {
            let xs = ranges[0][0]..=ranges[0][1];
            let ys = ranges[1][0]..=ranges[1][1];
            let zs = ranges[2][0]..=ranges[2][1];

            let points = zs.flat_map(|z| {
                let xs = &xs;
                ys.clone().flat_map(move |y| xs.clone().map(move |x| (x, y, z)))
            }).into_iter().collect::<HashSet<(i32, i32, i32)>>();

            if *status {
                unique.extend(&points);
            }
            else {
                unique = &unique - &points;
            }
        }
    }
    
    unique.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day22_input_test.txt";

    #[test]
    fn day22_part1_test() {
        let cuboids = parse_cuboids(FILE);
        
        assert_eq!(count_cuboids_on(&cuboids, 50), 590784);
    }

    #[test]
    fn day22_part2_test() {
        
        //assert_eq!(, 2758514936282235);
    }
}

