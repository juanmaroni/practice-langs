// Advent of Code 2022
// Day 1: Calorie Counting

use std::io::BufRead;
use itertools::Itertools;
use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day01_input.txt";

pub fn print_answers() {
    let mut day = Day::new(1, FILE.to_string());

    let input = parse_input(&day);
    let iter_sums = sum_and_sort_calories_by_elf(&input);
    let top3 = get_top3(iter_sums);

    day.first_answer = Some(Answer::Num(get_higher_calories_one_elf(&top3)));
    day.second_answer = Some(Answer::Num(sum_top3(&top3)));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> Vec<Vec<u32>> {
    let mut calories: Vec<Vec<u32>> = Vec::new();
    let mut elf: Vec<u32> = Vec::new();

    for line in day.read_file().lines() {
        let content = line.unwrap();

        if !content.is_empty() {
            elf.push(content.parse::<u32>().unwrap());
        } else {
            calories.push(elf);
            elf = Vec::new();
        }
    }

    // Add collected data before EOF
    calories.push(elf);

    calories
}

// Part 1
fn get_higher_calories_one_elf(calorie_sums: &Vec<u32>) -> u64 {
    calorie_sums[0] as u64
}

// Part 2
fn sum_top3(calorie_sums: &Vec<u32>)-> u64 {
    calorie_sums.iter()
        .sum::<u32>() as u64
}

// Create a sorted iterator with the calorie sum of each elf
fn sum_and_sort_calories_by_elf<'a>(calories: &'a Vec<Vec<u32>>) -> impl Iterator<Item = u32> + 'a {
    calories.iter()
        .map(|elf| elf.iter().sum::<u32>())
        .sorted_by(|a, b| Ord::cmp(b, a))
}

// Get Top 3 higher calorie sums to solve both parts
fn get_top3<'a>(calorie_sums: impl Iterator<Item = u32> + 'a) -> Vec<u32> {
    calorie_sums.take(3)
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day01_input_test.txt";

    #[test]
    fn day01_part1_test() {
        let mut day = Day::new(1, FILE.to_string());

        let input = parse_input(&day);
        let iter_sums = sum_and_sort_calories_by_elf(&input);
        let top3 = get_top3(iter_sums);
        let ans = get_higher_calories_one_elf(&top3);

        assert_eq!(ans, 24000);

        day.first_answer = Some(Answer::Num(ans));
    }

    #[test]
    fn day01_part2_test() {
        let mut day = Day::new(1, FILE.to_string());

        let input = parse_input(&day);
        let iter_sums = sum_and_sort_calories_by_elf(&input);
        let top3 = get_top3(iter_sums);
        let ans = sum_top3(&top3);

        assert_eq!(ans, 45000);

        day.second_answer = Some(Answer::Num(ans));
    }
}
