use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::HashMap;
use regex::Regex;

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

// First time using Regex crate
pub fn get_bags(filename: &str) -> HashMap<String, HashMap<String, u8>> {
    let re_parent = Regex::new(r"^(?P<parent_bag>[\w]+ [\w]+)").unwrap();
    let re_children = Regex::new(r"(?P<capacity>[0-9]+) (?P<child_bag>[\w]+ [\w]+)").unwrap();
    let mut bags_collection: HashMap<String, HashMap<String, u8>> = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(bags) = line {
                let parent = re_parent.find(&bags).unwrap().as_str().to_string();
                let mut map_children_bags: HashMap<String, u8> = HashMap::new();

                for ch in re_children.captures_iter(&bags) {
                    map_children_bags.insert(Some(&ch[2]).unwrap().to_string(), Some(&ch[1]).unwrap().parse::<u8>().unwrap());
                }
            
                bags_collection.insert(parent, map_children_bags);
            }
        }
    }

    //println!("{:?}", bags_collection);
    bags_collection
}

pub fn read_instructions(filename: &str) -> Vec<(String, i16)> {
    let mut instructions: Vec<(String, i16)> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instr) = line {
                let args: Vec<&str> = instr.split_whitespace().collect();
                instructions.push((args[0].to_string(), args[1].parse::<i16>().unwrap()));
            }
        }
    }

    instructions
}

pub fn get_numbers(filename: &str) -> Vec<u64> {
    let mut numbers: Vec<u64> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(number) = line {
                let n: u64 = number.parse::<u64>().unwrap();
                numbers.push(n);
            }
        }
    }

    numbers
}

pub fn generate_matrix(filename: &str) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = Vec::new();

    // I will fill borders with '_' to check bounds.
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(row) = line {
                let mut m_row: Vec<char> = Vec::new();
                m_row.push('_');
                
                for r in row.chars() {
                    m_row.push(r);
                }

                m_row.push('_');
                matrix.push(m_row);
            }
        }
    }

    let mut m_blank: Vec<char> = Vec::new();

    for _ in 0..matrix[0].len() {
        m_blank.push('_');
    }

    matrix.insert(0, m_blank.clone());
    matrix.push(m_blank);

    matrix
}

pub fn get_ship_instructions(filename: &str) -> Vec<(char, i16)> {
    let mut instructions: Vec<(char, i16)> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(instr) = line {
                let instr_chars: Vec<char> = instr.chars().collect();
                let dir = instr_chars[0];
                let movement: String = instr_chars[1..].iter().collect();
                instructions.push((dir, movement.parse::<i16>().unwrap()));
            }
        }
    }

    instructions
}

pub fn get_bus_schedule(filename: &str) -> (u32, Vec<u16>) {
    let mut schedule: (u32, Vec<u16>) = (0, Vec::new());

    if let Ok(lines) = read_lines(filename) {
        let mut is_timestamp: bool = true;

        for line in lines {
            if let Ok(note) = line {
                match is_timestamp {
                    true => {
                        schedule.0 = note.parse::<u32>().unwrap();
                        is_timestamp = false;
                    },
                    false => {
                        schedule.1 = note.split(',').filter(|id| id.parse::<u16>().is_ok()).map(|id| id.parse::<u16>().unwrap()).collect();
                        //is_timestamp = true; // Not necessary, because there are only 2 lines.
                    }
                }
            }
        }
    }

    schedule
}

pub fn get_bus_ids(filename: &str) -> Vec<u16> {
    let mut ids: Vec<u16> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines.skip(1) {
            if let Ok(note) = line {
                // I will replace 'x' with '0' so I can manage it easily.
                ids = note.replace("x", "0").split(',').map(|id| id.parse::<u16>().unwrap()).collect();
            }
        }
    }
    
    ids
}
