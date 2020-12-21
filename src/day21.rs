// Advent of Code 2020: December, 21
// Day 21: Allergen Assessment

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use crate::manage_input::get_food;

pub fn answers_day21() -> (u16, u64) {
    let filepath: &str = "inputs/day21_input.txt";
    let foods = get_food(filepath);
    find_no_allergens(foods);
    (0, 0)
}

fn find_no_allergens(foods: Vec<(Vec<String>, Vec<String>)>) {
    let unique_ingredients: HashSet<String> = HashSet::from_iter(foods.iter().flat_map(| v| v.0.iter()).cloned());
    let unique_allergens: HashSet<String> = HashSet::from_iter(foods.iter().flat_map(| v| v.1.iter()).cloned());

    
        

    println!("{:?}", unique_ingredients);
    println!("{:?}", unique_allergens);
}

