// Advent of Code 2020: December, 7
// Day 7: Handy Haversacks

use crate::manage_input;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn answers_day7() -> (u16, u16) {
    let filepath: &str = "inputs/day07_input.txt";
    count_shiny_gold(filepath);

    (0, 0)
}

// Bad at graphs, send help.
fn count_shiny_gold(filepath: &str) -> usize {
    let bags_collection: HashMap<String, HashMap<String, u8>> = manage_input::get_bags(filepath);
    let mut count: usize = 0;

    let mut visited: HashSet<String> = HashSet::new();
    visited.insert("shiny gold".to_string());
    
    println!("{:?}\n", bags_collection);
    
    for container in bags_collection.keys() {
        for bag in bags_collection.get(container).unwrap().keys() {
            if visited.contains(bag) {
                println!("Container {:?}, contains {:?}", container, bag);
                visited.insert(container.to_string());
            }
        }
        
    }

    println!("\n{:?}\n", visited.len());
    0
}
