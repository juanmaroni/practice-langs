// Advent of Code 2022
// Day 7: No Space Left On Device

use std::io::BufRead;
use std::collections::HashMap;
use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day07_input.txt";

pub fn print_answers() {
    let mut day = Day::new(7, FILE.to_string());

    let fs_tree = parse_input(&day);

    day.first_answer = Some(Answer::Num(sum_sizes_below_100000(fs_tree)));
    day.second_answer = Some(Answer::Num(0));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> HashMap<String, Vec<(u64, String)>> {
    // Key: directory.
    // Values: tuple with directories/files and their size (0 for directories).
    let mut fs_tree: HashMap<String, Vec<(u64, String)>> = HashMap::new();
    // Keeping track of current directory
    let mut track_dirs: Vec<String> = Vec::new();
    // String for "cd .."
    let cd_dotdot = String::from("$ cd ..");

    for line in day.read_file().lines() {
        let content = line.unwrap();
        let cmd = content[..4].to_string();
        
        if content == cd_dotdot {
            track_dirs.pop();
        } else if &cmd == "$ cd" {
            let dir = content[5..].to_string();
            let value: Vec<(u64, String)> = Vec::new();
            track_dirs.push(dir.clone());
            fs_tree.insert(dir, value);
        } else if &cmd == "$ ls" {
            // This doesn't matter
            continue;
        } else {
            let inside = content.split_whitespace().collect::<Vec<&str>>();
            let size = inside[0].parse::<u64>();
            let size = match size.is_err() {
                true => 0,
                false => size.unwrap(),
            };

            fs_tree.get_mut(&track_dirs[track_dirs.len() - 1]).unwrap().push((size, inside[1].to_string()));
        }
    }
    
    fs_tree
}

// Part 1
fn sum_sizes_below_100000(fs_tree: HashMap<String, Vec<(u64, String)>>) -> u64 {
    fs_tree.iter()
        .map(|(k, _)| sum_filesizes(k, &fs_tree))
        .filter(|sum| *sum <= 100000)
        .sum() //1011567 fails
}

// Part 2
fn calc_second_answer(fs_tree: HashMap<String, Vec<(u64, String)>>) -> u32 {
    todo!()
}

// Helping functions for Part 1
fn sum_filesizes(key: &String, fs_tree: &HashMap<String, Vec<(u64, String)>>) -> u64 {
    let mut total_size = 0;

    for v in fs_tree.get(key) {
        total_size += recursive_sum_filesize(v, fs_tree);
    }

    total_size
}

fn recursive_sum_filesize(content: &Vec<(u64, String)>, fs_tree: &HashMap<String, Vec<(u64, String)>>) -> u64 {
    let mut total_size = 0;

    for (s, n) in content {
        if *s == 0 {
            total_size += recursive_sum_filesize(fs_tree.get(n).unwrap(), fs_tree)
        } else {
            total_size += s
        }
    }

    total_size
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day07_input_test.txt";

    #[test]
    fn day07_part1_test() {
        let mut day = Day::new(7, FILE.to_string());
        let fs_tree = parse_input(&day);
        let ans = sum_sizes_below_100000(fs_tree);

        assert_eq!(ans, 95437);

        day.first_answer = Some(Answer::Num(ans));
    }

    #[test]
    fn day07_part2_test() {
        let mut day = Day::new(7, FILE.to_string());
        

        assert_eq!(5, 5);

        day.second_answer = Some(Answer::Num(0));
    }
}
