// Advent of Code 2020: December, 1
// Day 1: Report Repair

use crate::manage_input;

pub fn answer_day1_part1() -> i32 {
    find_year_sum_part1(2020)
}

pub fn answer_day1_part2() -> i32 {
    find_year_sum_part2(2020)
}

fn find_year_sum_part1(year: i16) -> i32 {
    let numbers: Vec<i16> = manage_input::numbers_from_file("inputs/day01_input.txt", year);
    let mut cp_numbers: Vec<i16> = numbers.clone();
    
    // My idea is to use the vector and a clone of it.
    // Iterating over the numbers from original vector, the loop checks if the sum of the number from the original and
    // any number from the clon equals to the year number. After each iteration, the first number from the clon is removed
    // because it has already been tried.
    for n in numbers {
        cp_numbers.remove(0);

        for n2 in &cp_numbers {
            if n + n2 == 2020 {
                // println!("Numbers: {} and {}", n, n2);

                // This was tricky, I tried multiplying and then casting to i32 (even i64)
                // and the compiler panicked because it was causing overflow.
                // Turned out that I needed to cast every value BEFORE multiplying.
                return i32::from(n) * i32::from(*n2);
            }
        }
    }

    -1
}

fn find_year_sum_part2(year: i16) -> i32 {
    let numbers: Vec<i16> = manage_input::numbers_from_file("inputs/day01_input.txt", year);
    let mut cp_numbers: Vec<i16> = numbers.clone();

    // It's almost the same, I just need to clone the first clone and loop, removing numbers.
    for n in numbers {
        cp_numbers.remove(0);
        let mut cp2_numbers: Vec<i16> = cp_numbers.clone();

        for n2 in &cp_numbers {
            cp2_numbers.remove(0);
            
            for n3 in &cp2_numbers {
                if n + n2 + n3 == 2020 {
                    return i32::from(n) * i32::from(*n2) * i32::from(*n3);
                }
            }
        }
    }

    -1
}
