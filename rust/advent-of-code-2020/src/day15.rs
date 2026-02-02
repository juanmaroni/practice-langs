// Advent of Code 2020: December, 15
// Day 15: Rambunctious Recitation

use std::collections::BTreeMap;
use crate::manage_input::get_game_nums;

pub fn answers_day15() -> (usize, usize) {
    let filepath: &str = "inputs/day15_input.txt";
    let part1 = play_game(get_game_nums(filepath), 2020);
    let part2 = play_game_v2(get_game_nums(filepath), 30000000);
    
    (part1, part2)
}

// Too slow for big limits.
fn play_game(nums: Vec<usize>, limit: usize) -> usize {
    let mut turn: usize = 1;
    let mut passed: BTreeMap<usize, usize> = BTreeMap::new();

    for n in nums {
        passed.insert(turn, n);
        turn += 1;
    }

    loop {
        let mut kv: (usize, usize) = (0, 0);
        let prev_turn = turn - 1;
        let prev_num = passed.get(&prev_turn).unwrap();

        if passed.values().take(prev_turn - 1).find(|v| v == &prev_num) == None {
            kv = (turn, 0);
        }
        else {
            kv = (turn, prev_turn - passed.keys().rev().skip(1).find(|k| passed.get(k).unwrap() == prev_num).unwrap());
        }

        if kv != (0, 0) {
            passed.insert(kv.0, kv.1);
        }

        if turn == limit {
            break;
        }
        
        turn += 1;
    }

    *passed.get(&limit).unwrap()
}

// Not my idea. Learning to optimiza will take time. I learned about Van Eck sequence.
fn play_game_v2(nums: Vec<usize>, limit: usize) -> usize {
    let mut positions: BTreeMap<usize, usize> = BTreeMap::new();
    let starting_pos = nums.len() - 1;

    for (i, n) in nums.iter().enumerate().take(starting_pos) {
        positions.insert(*n, i);
    }

    let mut current: usize = *nums.last().unwrap();

    for current_idx in (starting_pos)..(limit - 1) {
        let prev: Option<usize> = positions.insert(current, current_idx);
        current = prev.map(|x| current_idx - x).unwrap_or(0);
    }

    current
}
