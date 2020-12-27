// Advent of Code 2020: December, 9
// Day 9: Encoding Error

use crate::manage_input::parse_numbers_u64;

pub fn answers_day9() -> (u64, u64) {
    let filepath: &str = "inputs/day09_input.txt";
    let input = parse_numbers_u64(filepath);
    let len: usize = 25;
    let first_missing = check_sums(&input, len);

    (first_missing, find_weakness(&input, first_missing))
}

fn check_sums(nums: &Vec<u64>, len: usize) -> u64 {
    for window in nums.windows(len + 1).skip(len) {
        let mut i = 1;
        let mut sums: Vec<u64> = Vec::new();
        let find: u64 = window[len];
        
        for n in window[..len].into_iter() {
            for m in window[i..len].into_iter() {
                if n != m {
                    sums.push(n + m);
                }
            }

            i += 1;
        }

        if !sums.contains(&find) {
            return find;
        }
    }

    0   // Nothing found.
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

        let m = i + 1;
        
        if sum + nums[m] == missing {
            slice.push(nums[m]);
            break;
        }
        else if sum + nums[m] > missing {
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
