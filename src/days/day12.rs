// Advent of Code 2021
// Day 12: Passage Pathing

use crate::input_handler::parse_cave_paths;
use std::collections::{HashMap, HashSet};

const FILE: &str = "inputs/day12_input.txt";

pub fn day12_answer() {
    let mut cave_paths = parse_cave_paths(FILE);

    println!("Day 12, part 1: {}", 0);
    println!("Day 12, part 2: {}\n", 0);
}

fn path_builder(paths: &mut HashMap<String, Vec<String>>, mut current: String, mut counter: u16) {
    let mut little_caves: HashSet<String> = HashSet::new();
    let next_points = paths.get(&current).expect("Some error");

    for p in next_points {
        if p.chars().all(|c| c.is_ascii_lowercase()) {
            little_caves.insert(String::from(p));
        }
        else if p == &String::from("end") {
            counter += 1;
            return;
        }

        //path_builder(paths, p, n_paths);
    }

}

fn count_paths(paths: &mut HashMap<String, Vec<String>>) -> u16 {
    let mut n_paths = 0;

    path_builder(paths, String::from("start"), n_paths);

    println!("{:?}", n_paths);
    n_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day12_input_test.txt";

    #[test]
    fn day12_part1_test() {
        let mut cave_paths = parse_cave_paths(FILE);
        //count_paths(&mut cave_paths);
        
        //assert_eq!(path_builder(cave_paths), 10);
    }

    #[test]
    fn day12_part2_test() {
        //let heightmap = parse_heightmap(FILE);
        //

        //assert_eq!(basins(&mut heightmap), 1134);
    }
}
