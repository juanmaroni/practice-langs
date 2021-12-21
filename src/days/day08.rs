// Advent of Code 2021
// Day 8: Seven Segment Search

use crate::input_handler::parse_patterns_outputs;
use std::collections::{HashMap, HashSet};

const FILE: &str = "inputs/day08_input.txt";

pub fn day08_answer() {
    let patterns_outputs = parse_patterns_outputs(FILE);

    println!("Day 8, part 1: {}", count_unique(&patterns_outputs));
    
    println!("Day 8, part 2: {}\n", decode_patterns(&patterns_outputs));
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

fn map_unique(patterns: &[String; 10]) -> HashMap<u8, HashSet<char>> {
    let mut map: HashMap<u8, HashSet<char>> = HashMap::new();
    let mut patterns = patterns.to_vec();

    for p in patterns.iter().filter(|s| s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7) {
        let p_chars = p.chars().collect::<HashSet<char>>();

        if p_chars.len() == 2 {
            map.insert(1, p_chars);
        }
        else if p_chars.len() == 4 {
            map.insert(4, p_chars);
        }
        else if p_chars.len() == 3 {
            map.insert(7, p_chars);
        }
        else if p_chars.len() == 7 {
            map.insert(8, p_chars);
        }
    }

    patterns.retain(|s| s.len() == 5 || s.len() == 6);

    for p in patterns.iter() {
        let p_chars = p.chars().collect::<HashSet<char>>();

        // 2, 3, 5 => 5 segments
        if p_chars.len() == 5 {
            if &p_chars.intersection(&map.get(&1).unwrap()).collect::<Vec<_>>().len() == &2 {
                map.insert(3, p_chars.clone());
            }
            else if &p_chars.intersection(&map.get(&4).unwrap()).collect::<Vec<_>>().len() == &3 {
                map.insert(5, p_chars.clone());
            }
            else {
                map.insert(2, p_chars.clone());
            }
        }
        // 0, 6, 9 => 6 segments
        else if p_chars.len() == 6 {
            if &p_chars.intersection(&map.get(&4).unwrap()).collect::<Vec<_>>().len() == &4 {
                map.insert(9, p_chars.clone());
            }
            else if &p_chars.intersection(&map.get(&1).unwrap()).collect::<Vec<_>>().len() == &2 {
                map.insert(0, p_chars.clone());
            }
            else {
                map.insert(6, p_chars.clone());
            }
        }
    }

    map
}

fn decode_patterns(patterns_outputs: &Vec<([String; 10], [String; 4])>) -> u32 {
    let mut total_output_values = 0;

    for (patterns, outputs) in patterns_outputs {
        let mut str_output = String::new();
        let map_patterns: HashMap<u8, HashSet<char>> = map_unique(patterns);

        for o in outputs {
            let o_chars = o.chars().collect::<HashSet<char>>();

            for (k, p) in map_patterns.iter() {
                if &o_chars == p {
                    str_output.push(char::from_digit(*k as u32, 10).unwrap());
                    break;
                }
            }
        }

        total_output_values += str_output.parse::<u32>().unwrap();
    }

    total_output_values
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

        assert_eq!(decode_patterns(&patterns_outputs), 61229);
    }
}
