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

pub fn parse_expense_report(filename: &str, year: u16) -> Vec<u16> {
    let mut expense_report: Vec<u16> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(number) = line {
                let n: u16 = number.parse::<u16>().unwrap_or(0);

                // Numbers can't be greater than the year, so I will exclude them.
                // Probably not necessary, if every input contains numbers less than or equal to the year
                if n <= year {
                    expense_report.push(n);
                }
            }
        }
    }

    expense_report
}

pub fn parse_lines(filename: &str) -> Vec<String>{
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

pub fn parse_grid(filename: &str) -> String {
    let mut grid: String = String::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(path) = line {
                grid += &(path + "\n");
            }
        }
    }

    grid
}

pub fn parse_passports(filename: &str) -> Vec<HashMap<String, String>> {
    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let mut passport_fields: HashMap<String, String> = HashMap::new();
    
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

pub fn parse_form_answers(filename: &str) -> (Vec<(u16, Vec<char>)>, u16) {
    let mut answers_by_people: Vec<(u16, Vec<char>)> = Vec::new();
    let mut n_people: u16 = 0;
    let mut answers: Vec<char> = Vec::new();
    let mut everyone_answered: u16 = 0;

    if let Ok(lines) = read_lines(filename) {
        // This will merge every answer from a group and then I will use it to count
        // their common answers.
        let mut merge_answers = String::new();

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
pub fn parse_bags(filename: &str) -> HashMap<String, HashMap<String, u8>> {
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

// Didn't check the input before writing the parser.
// I thought it was only ONE mask for MULTIPLE memory operations.
// So I had to rewrite. And probably overdid it.
pub fn get_memory_data(filename: &str) -> Vec<(HashMap<usize, char>, HashMap<u64, u64>)> {
    let mut piece: Vec<(HashMap<usize, char>, HashMap<u64, u64>)> = Vec::new();
    let mut bitmask: HashMap<usize, char> = HashMap::new();
    let mut mem_ops: HashMap<u64, u64>  = HashMap::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(data) = line {
                if &data[0..2] == "ma" {
                    if !bitmask.is_empty() && !mem_ops.is_empty() {
                        piece.push((bitmask.clone(), mem_ops.clone()));
                        bitmask.clear();
                        mem_ops.clear();
                    }

                    for (i, b) in data.replace("mask = ", "").chars().rev().enumerate() {
                        if b.is_digit(2) {
                            bitmask.insert(i, b);
                        }
                    }
                }
                else {
                    let mut pair: Vec<u64> = Vec::new();

                    for n in data.replace("mem[", "").replace("]", "").split(" = ").map(|n| n.to_string().parse::<u64>().unwrap()) {
                        pair.push(n);
                    }

                    mem_ops.insert(pair[0], pair[1]);
                }
            }
        }
    }

    piece
}

pub fn get_game_nums(filename: &str) -> Vec<usize> {
    let mut nums: Vec<usize> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(num) = line {
                nums = num.split(",").map(|n| n.to_string().parse::<usize>().unwrap()).collect();
            }
        }
    }

    nums
}

// Reading the notes and dividing in three parts: rules, my ticket and nearby tickets.
pub fn read_notes(filename: &str) -> Vec<Vec<String>> {
    let mut notes: Vec<Vec<String>> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        let mut part: Vec<String> = Vec::new();

        for line in lines {
            if let Ok(note) = line {
                if !note.is_empty() {
                    part.push(note);
                }
                else {
                    notes.push(part);
                    part = Vec::new();
                }
            }
        }
    }

    notes
}

pub fn get_cubes(filename: &str) -> Vec<Vec<bool>> {
    let mut cubes: Vec<Vec<bool>> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(cube) = line {
                let row: Vec<bool> = cube.chars().map(|c| c == '#').collect();
                cubes.push(row);
            }
        }
    }

    cubes
}

pub fn get_expressions(filename: &str) -> Vec<String> {
    let mut expressions: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(e) = line {
                expressions.push(e.replace(" ", ""));
            }
        }
    }

    expressions
}

pub fn get_rules_and_messages(filename: &str) -> (Vec<String>, Vec<String>) {
    let mut rules_and_messages: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(l) = line {
                rules_and_messages.push(l);
            }
        }
    }

    // The input has two parts and I use chunks to separate them by the empty line.
    let input_chunks: Vec<&[String]> = rules_and_messages
    .chunks(rules_and_messages.iter().position(|l| l.is_empty()).unwrap()).collect();

    let rules = input_chunks[0].to_vec();
    let messages = input_chunks[1][1..].to_vec();

    (rules, messages)
}

pub fn get_tiles(filename: &str) -> HashMap<u32, Vec<String>> {
    let mut tiles: HashMap<u32, Vec<String>> = HashMap::new();
    let mut k: u32 = 0;
    let mut input: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(mut t) = line {
                if t.contains("Tile") {
                    t.retain(|c| c.is_digit(10));
                    k = t.parse::<u32>().unwrap();
                }
                else if t.is_empty() {
                    tiles.insert(k, input);
                    input = Vec::new();
                }
                else {
                    input.push(t);
                }
            }
        }
    }
    
    tiles
}

pub fn get_food(filename: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let mut foods: Vec<(Vec<String>, Vec<String>)> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(f) = line {
                let clean = f.replace(&['(', ')', ','][..], "");
                let sep: Vec<String> = clean.split(" contains ").map(|s| s.to_string()).collect();
                foods.push((sep[0].split(" ").map(|s| s.to_string()).collect(), sep[1].split(" ").map(|s| s.to_string()).collect()));
            }
        }
    }
    
    foods
}

pub fn get_decks(filename: &str) -> Vec<Vec<u8>> {
    let mut decks: Vec<Vec<u8>> = Vec::new();
    let mut cards: Vec<u8> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(card) = line {
                if card.contains("P") {
                    continue;
                }
                else if card.is_empty() {
                    decks.push(cards);
                    cards = Vec::new();
                }
                else {
                    cards.push(card.parse::<u8>().unwrap());
                }
            }
        }
    }

    decks
}

pub fn get_cups(filename: &str) -> Vec<u8> {
    let mut cups: Vec<u8> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(c) = line {
                cups = c.chars().map(|label| label.to_digit(10).unwrap() as u8).collect();
            }
        }
    }

    cups
}

pub fn get_hexa_tiles(filename: &str) -> Vec<Vec<String>> {
    let mut hexa_tiles: Vec<Vec<String>> = Vec::new();
    let re = Regex::new("(se)|(sw)|(ne)|(nw)|(w)|(e)+").unwrap();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(t) = line {
                let mut tiles_dir: Vec<String> = Vec::new();

                for c in re.captures_iter(&t) {
                    tiles_dir.push(c[0].to_string());
                }
                
                hexa_tiles.push(tiles_dir);
            }
        }
    }

    //println!("{:?}", hexa_tiles);
    hexa_tiles
}

pub fn get_public_keys(filename: &str) -> (u32, u32) {
    let mut keys: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        // Only two keys.
        for line in lines.take(2) {
            if let Ok(k) = line {
                keys.push(k.parse::<u32>().unwrap());
            }
        }
    }

    (keys[0], keys[1])
}
