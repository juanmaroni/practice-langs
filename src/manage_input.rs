use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;

// From documentation "Rust by Example"
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P:AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn numbers_from_file(filename: &str, year: i16) -> Vec<i16> {
    let mut numbers: Vec<i16> = Vec::new();

    // Assumptions:
    // - The input has only positive integers (unsigned int), so numbers higher than 2020 are out.
    // - u16 and i16 are enough fo the few years to come.
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(number) = line {
                let n: i16 = number.parse::<i16>().unwrap();    // TODO: I could handle error on this

                // Numbers can't be greater than the year, so I will exclude them.
                if n <= year {
                    numbers.push(n);
                }
            }
        }
    }

    numbers
}

pub fn passwords_from_file(filename: &str) -> Vec<String>{
    let mut passwords: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(pw) = line {
                passwords.push(pw);
            }
        }
    }

    passwords
}

pub fn grid_from_file(filename: &str) -> (String, usize) {
    let mut grid: String = "".to_owned();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(path) = line {
                grid += &(path + "\n");
            }
        }
    }

    let len_pattern: usize = grid.find('\n').unwrap();

    (grid, len_pattern)
}

pub fn read_passports(filename: &str) -> Vec<HashMap<String, String>> {
    let mut passport_fields: HashMap<String, String> = HashMap::new();
    let mut passports: Vec<HashMap<String, String>> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(fields) = line {
                if !fields.is_empty() {
                    for field in fields.split(' ') {
                        let pair: Vec<&str> = field.split(':').collect();
                        passport_fields.insert(String::from(pair[0]), String::from(pair[1]));
                    }
                }
                else {
                    if !passport_fields.is_empty() {
                        passports.push(passport_fields);
                        passport_fields = HashMap::new();
                    }
                }
            }
        }
    }

    passports
}

pub fn read_answers(filename: &str) -> (Vec<(u16, Vec<char>)>, u16) {
    let mut answers_by_people: Vec<(u16, Vec<char>)> = Vec::new();
    let mut n_people: u16 = 0;
    let mut answers: Vec<char> = Vec::new();
    let mut everyone_answered: u16 = 0;

    if let Ok(lines) = read_lines(filename) {
        // This will merge every answer from a group and then I will use it to count
        // their common answers
        let mut merge_answers: String = "".to_owned();

        for line in lines {
            if let Ok(person) = line {
                if !person.is_empty() {
                    n_people += 1;
                    merge_answers = merge_answers + &person;

                    for a in person.chars() {
                        if !answers.contains(&a) {
                            answers.push(a);
                        }
                    }
                }
                else {
                    if !answers.is_empty() {
                        for a in &answers {
                            let c: u16 = merge_answers.matches(*a).count() as u16;

                            if c == n_people {
                                everyone_answered += 1;
                            }
                        }
                        
                        answers_by_people.push((n_people, answers));

                        // Restarting variables for next group.
                        merge_answers = "".to_string();
                        answers = Vec::new();
                        n_people = 0;
                    }
                }
            }
        }
    }

    // (Part1, Part2)
    (answers_by_people, everyone_answered)
}
