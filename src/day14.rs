// Advent of Code 2020: December, 14
// Day 14: Docking Data

use std::collections::HashMap;
use crate::manage_input::get_memory_data;

pub fn answers_day14() -> (u64, u64) {
    let filepath: &str = "inputs/day14_input.txt";
    let part1 = sum_unique_masks(get_memory_data(filepath));
    //println!("{:?}", part1);
    (part1, 0)
}

fn sum_unique_masks(data: Vec<(HashMap<usize, char>, HashMap<u64, u64>)>) -> u64 {
    let mut sum: u64 = 0;
    // Keeping track of used memory address.
    let mut mem_passed: Vec<u64> = Vec::new();

    for d in data.iter().rev() {
        let bitmask = &d.0;
        let mem_ops = &d.1;
        
        for k in mem_ops.keys() {
            if !mem_passed.contains(k) {
                let v: &u64 = mem_ops.get(k).unwrap();
                sum += apply_mask(bitmask.clone(), *v);
                mem_passed.push(*k);
            }
            
        }
    }

    sum
}

fn apply_mask(bitmask: HashMap<usize, char>, n: u64) -> u64 {
    let bm = &bitmask;
    let mut bin_bits: Vec<char> = format!("{:064b}", n).chars().rev().collect();

    for k in bm.keys() {
        bin_bits[*k as usize] = 
        *bm.get(k).unwrap();
    }

    let bin: String = bin_bits.into_iter().rev().collect();

    u64::from_str_radix(&bin[..], 2).unwrap()
}
