// Advent of Code 2020: December, 25
// Day 25: Combo Breaker

use crate::manage_input::get_public_keys;

const DIV_VALUE: u32 = 20_201_227;
const SUBJECT_NUMBER: u32 = 7;

pub fn answers_day25() -> u64 {
    let filepath: &str = "inputs/day25_input.txt";
    let keys = get_public_keys(filepath);
    let pk_card = keys.0;
    let pk_door = keys.1;
    
    // PK variables are interchangeables.
    calc_encryption_key(pk_door, calc_loopsize(pk_card))
}

fn calc_loopsize(public_key: u32) -> u32 {
    let mut loopsize: u32 = 0;
    let mut val = 1;

    while val != public_key {
        val = (val * SUBJECT_NUMBER) % DIV_VALUE;
        loopsize += 1;
    }
    
    loopsize
}

fn calc_encryption_key(public_key: u32, loopsize: u32) -> u64 {
    let mut encryption_key: u64 = 1;

    for _ in 0..loopsize {
        encryption_key = (encryption_key * public_key as u64) % DIV_VALUE as u64;
    }

    encryption_key
}
