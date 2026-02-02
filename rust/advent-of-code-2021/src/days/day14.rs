// Advent of Code 2021
// Day 14: Extended Polymerization

use crate::input_handler::parse_polymer_instructions;
use std::collections::HashMap;

const FILE: &str = "inputs/day14_input.txt";

pub fn day14_answer() {
    let (template, pair_insertions) = parse_polymer_instructions(FILE);
    
    let map_elements = count_elements(&template, &pair_insertions, 10);
    let max_element = map_elements.values().max().unwrap();
    let min_element = map_elements.values().min().unwrap();
    println!("Day 14, part 1: {}", max_element - min_element);

    let map_elements = count_elements(&template, &pair_insertions, 40);
    let max_element = map_elements.values().max().unwrap();
    let min_element = map_elements.values().min().unwrap();
    println!("Day 14, part 2: {}\n", max_element - min_element);
}

fn upsert_pair(pairs: &mut HashMap<String, usize>, p: String, v: usize) {
    if pairs.contains_key(&p) {
        *pairs.get_mut(&p).unwrap() += v;
    }
    else {
        pairs.insert(p, v);
    }
}

fn upsert_elem(elems: &mut HashMap<char, usize>, e: char, v: usize) {
    if elems.contains_key(&e) {
        *elems.get_mut(&e).unwrap() += v;
    }
    else {
        elems.insert(e, v);
    }
}

fn count_elements(template: &String, pair_insertions: &HashMap<String, char>, steps: usize) -> HashMap<char, usize> {
    let mut pairs: HashMap<String, usize> = HashMap::new();
    let mut elems: HashMap<char, usize> = HashMap::new();

    let template_chars = template.chars().collect::<Vec<char>>();

    for c in template_chars.iter() {
        upsert_elem(&mut elems, *c, 1);
    }

    for p in template_chars.windows(2).map(|p| p.into_iter().collect()) {
        upsert_pair(&mut pairs, p, 1);
    }

    for _ in 0..steps {
        for (pair, count) in pairs.clone() {
            let pair_chars = pair.chars().collect::<Vec<char>>();
            let new_elem = pair_insertions.get(&pair).unwrap();

            upsert_elem(&mut elems, new_elem.clone(), count);
            
            *pairs.get_mut(&pair).unwrap() -= count;

            upsert_pair(&mut pairs, vec!(pair_chars[0], new_elem.clone()).iter().collect::<String>(), count);
            upsert_pair(&mut pairs, vec!(*new_elem, pair_chars[1]).iter().collect::<String>(), count);
        }
    }

    elems
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day14_input_test.txt";

    #[test]
    fn day14_part1_test() {
        let (template, pair_insertions) = parse_polymer_instructions(FILE);
        let map_elements = count_elements(&template, &pair_insertions, 10);
        let max_element = map_elements.values().max().unwrap();
        let min_element = map_elements.values().min().unwrap();
        
        assert_eq!(max_element - min_element, 1588);
    }

    #[test]
    fn day14_part2_test() {
        let (template, pair_insertions) = parse_polymer_instructions(FILE);
        let map_elements = count_elements(&template, &pair_insertions, 40);
        let max_element = map_elements.values().max().unwrap();
        let min_element = map_elements.values().min().unwrap();
        
        assert_eq!(max_element - min_element, 2188189693529);
    }
}
