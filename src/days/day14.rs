// Advent of Code 2021
// Day 14: Extended Polymerization

use crate::input_handler::parse_polymer_instructions;
use std::collections::HashMap;

const FILE: &str = "inputs/day14_input.txt";

pub fn day14_answer() {
    let (template, pairs) = parse_polymer_instructions(FILE);
    let map_elements = count_elements(run_n_steps(&template, &pairs, 10));
    let max_element = map_elements.values().max().unwrap();
    let min_element = map_elements.values().min().unwrap();

    println!("Day 14, part 1: {}", max_element - min_element);

    
}

fn apply_pairs(polymer: &String, pairs: &HashMap<String, char>) -> String {
    // Generate chunks of size 2 from polymer
    let polymer_chars= polymer.chars().collect::<Vec<char>>();
    let first_char = polymer_chars[0];
    let mut polymer_chunks: Vec<String> = Vec::new();

    for p in polymer_chars.windows(2) {
        polymer_chunks.push(p.into_iter().collect());
    }

    let mut new_polymer = String::from(first_char);

    for mut chunk in polymer_chunks {
        if pairs.contains_key(&chunk) {
            chunk.insert(1, *pairs.get(&chunk).unwrap());
            new_polymer.push_str(&chunk[1..]);
        }
        else {
            new_polymer.push_str(&chunk.chars().nth(1).unwrap().to_string());
        }
    }

    new_polymer
}

fn run_n_steps(template: &String, pairs: &HashMap<String, char>, steps: usize) -> String {
    let mut polymer = template.clone();

    for _ in 0..steps {
        polymer = apply_pairs(&mut polymer, &pairs);
    }

    polymer
}

fn count_elements(polymer: String) -> HashMap<char, usize> {
    let mut chars_appearances: HashMap<char, usize> = HashMap::new();

    for c in polymer.chars() {
        if chars_appearances.contains_key(&c) {
            *chars_appearances.get_mut(&c).unwrap() += 1;
        }
        else {
            chars_appearances.insert(c, 1);
        }
    }

    chars_appearances
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day14_input_test.txt";

    #[test]
    fn day14_part1_test() {
        let (template, pairs) = parse_polymer_instructions(FILE);
        let map_elements = count_elements(run_n_steps(&template, &pairs, 10));
        let max_element = map_elements.values().max().unwrap();
        let min_element = map_elements.values().min().unwrap();
        
        assert_eq!(max_element - min_element, 1588);
    }

    #[test]
    fn day14_part2_test() {
        
    }
}
