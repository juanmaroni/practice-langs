// Advent of Code 2020: December, 7
// Day 7: Handy Haversacks

use std::collections::{HashMap, HashSet};

use crate::manage_input::parse_bags;

type MapBagsCollection = HashMap<String, HashMap<String, u8>>;

pub fn answers_day7() -> (u16, u32) {
    let bags_collection: MapBagsCollection = parse_bags("inputs/day07_input.txt");

    (count_shiny_gold(&bags_collection),
    count_total_bags(&bags_collection, "shiny gold") - 1) // Discounting the shiny gold bag.
}

fn count_shiny_gold(bags_collection: &MapBagsCollection) -> u16 {
    let mut visited: HashSet<&str> = HashSet::new();
        
    for bag in bags_collection.keys() {
        if bags_collection[bag].iter().any(|(b, _)| contains_shiny_gold(&bags_collection, b)) {
            visited.insert(bag);
        }
    }

    visited.len() as u16
}

fn contains_shiny_gold(bags_collection: &MapBagsCollection, bag: &str) -> bool {
    bag == "shiny gold" || bags_collection[bag].iter()
                           .any(|(b, _)| contains_shiny_gold(bags_collection, b))
}

fn count_total_bags(bags_collection: &MapBagsCollection, bag: &str) -> u32 {
    1 + bags_collection[bag].iter()
        .map(|(b, n)| *n as u32 * count_total_bags(bags_collection, b))
        .sum::<u32>()
}
