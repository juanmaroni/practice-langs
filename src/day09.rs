// Advent of Code 2020: December, 9
// Day 9: Encoding Error

use crate::manage_input;

pub fn answers_day9() -> (u64, u64) {
    let filepath: &str = "inputs/day09_input.txt";
    let input = manage_input::get_numbers(filepath);
    let len: usize = 25;
    let first_missing = check_sums(&input, len);

    (first_missing, find_weakness(&input, first_missing))
}

fn check_sums(nums: &Vec<u64>, len: usize) -> u64 {
    // Just need the first, but I wanted to take all of them (if there were more than 1).
    let mut missing: Vec<u64> = Vec::new();

    for window in nums.windows(len + 1).skip(len) {
        let mut i = 1;
        let mut sums: Vec<u64> = Vec::new();
        let find: u64 = window[len];
        
        for n in window[..len].into_iter() {
            for n2 in window[i..len].into_iter() {
                if n != n2 {
                    sums.push(n + n2);
                }
            }

            i += 1;
        }

        if !sums.contains(&find) {
            missing.push(find);
        }
    }

    missing[0]
}

fn find_weakness(nums: &Vec<u64>, missing: u64) -> u64 {
    let mut slice: Vec<u64> = Vec::new();
    let mut next: usize = 0;
    let mut i = 0;

    while i < nums.len() - 1 {
        slice.push(nums[i]);
        let mut sum: u64 = 0;

        for n in &slice {
            sum += n;
        }
        
        if sum + nums[i + 1] == missing {
            slice.push(nums[i + 1] );
            break;
        }
        else if sum + nums[i + 1] > missing {
            slice = Vec::new();
            next += 1;
            i = next;
        }
        else {
            i += 1;
        }
    }

    slice.iter().min().unwrap() + slice.iter().max().unwrap()
}
