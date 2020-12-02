// Advent of Code 2020: December, 1
// Day 1: Report Repair

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn answer_day1_part1() -> i32 {
    find_year_sum_part1(2020)
}

pub fn answer_day1_part2() -> i32 {
    find_year_sum_part2(2020)
}

// From documentation
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P:AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn numbers_from_file(filename: &str, year: i16) -> Vec<i16> {
    let mut numbers: Vec<i16> = Vec::new();

    // Assumptions:
    // - The input has only positive integers (unsigned int), so numbers higher than 2020 are out.
    // - u16 and i16 are enough fo the few years to come.
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(number) = line {
                let n: i16 = number.parse::<i16>().unwrap();    // TODO: I could handle error on this

                // Numbers can't be greater than the year, so I will not include any number that
                // doesn't satisfy this.
                if n <= year {
                    numbers.push(n);
                }
            }
        }
    }

    numbers
}

fn find_year_sum_part1(year: i16) -> i32 {
    let numbers: Vec<i16> = numbers_from_file("inputs/day01_input.txt", year);
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
    let numbers: Vec<i16> = numbers_from_file("inputs/day1_input.txt", year);
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
