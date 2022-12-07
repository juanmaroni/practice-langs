// Advent of Code 2022
// Day 7: No Space Left On Device

use std::io::BufRead;
use std::collections::HashMap;
use aoc2022::{Day, Part, Answer};
use itertools::{Itertools};

const FILE: &str = "inputs/real/day07_input.txt";

pub fn print_answers() {
    let mut day = Day::new(7, FILE.to_string());

    let fs_tree = parse_input(&day);

    day.first_answer = Some(Answer::Num(sum_sizes_up_to_100000(fs_tree)));
    day.second_answer = Some(Answer::Num(0));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> HashMap<String, Vec<(u64, String)>> {
    // Key: path.
    // Values: file and size.
    let mut fs_tree: HashMap<String, Vec<(u64, String)>> = HashMap::new();
    // Keeping track of current directory
    let mut track_dirs: Vec<String> = Vec::new();
    // String for "cd .."
    let cd_dotdot = String::from("$ cd ..");

    for line in day.read_file().lines() {
        let content = line.unwrap();
        let cmd = content[..4].to_string();
        //let dir_path = track_dirs.iter().join("/");
        
        if content == cd_dotdot {
            track_dirs.pop();
        } else if &cmd == "$ cd" {
            let mut dir = content[5..].to_string();
            dir.push('/');
            track_dirs.push(dir);
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

            let dir_path = track_dirs.iter().join("");

            create_or_add_to_hashmap(dir_path, (size, inside[1].to_string()), &mut fs_tree);
        }
    }
    
    fs_tree
}

// Helping function for parsing
fn create_or_add_to_hashmap(k: String, v: (u64, String), hm: &mut HashMap<String, Vec<(u64, String)>>) {
    if !hm.contains_key(&k) {
        hm.insert(k, vec![v]);
    } else {
        hm.get_mut(&k).unwrap().push(v);
    }
}

// Part 1
fn sum_sizes_up_to_100000(fs_tree: HashMap<String, Vec<(u64, String)>>) -> u64 {
    fs_tree.iter()
        .map(|(k, _)| sum_filesizes(k, &fs_tree))
        .filter(|sum| *sum <= 100000)
        .sum() //1394611 high
}

// Part 2
fn calc_second_answer(fs_tree: HashMap<String, Vec<(u64, String)>>) -> u32 {
    todo!()
}

// Helping functions for Part 1
fn sum_filesizes(key: &String, fs_tree: &HashMap<String, Vec<(u64, String)>>) -> u64 {
    let mut total_size = 0;
    let mut key_cp = key.clone();

    for v in fs_tree.get(&key_cp) {
        total_size += recursive_sum_filesize(&key_cp, v, fs_tree);
    }

    //println!("{:?}", total_size);

    total_size
}

fn recursive_sum_filesize(key: &String, content: &Vec<(u64, String)>, fs_tree: &HashMap<String, Vec<(u64, String)>>) -> u64 {
    let mut total_size = 0;
    let mut key_cp = key.clone();

    for (s, n) in content {
        //println!("{:?}", key);
        if *s == 0 {
            key_cp.push_str(n);
            key_cp.push('/');

            let value = fs_tree.get(&key_cp);
            
            if value.is_some() {
                total_size += recursive_sum_filesize(&key_cp, value.unwrap(), fs_tree);
            }

            key_cp = key.clone();

        } else {
            total_size += s;
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
        let ans = sum_sizes_up_to_100000(fs_tree);

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
