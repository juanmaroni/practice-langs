// Advent of Code 2020: December, 4
// Day 4: Passport Processing

use crate::manage_input;

use std::collections::HashMap;

pub fn answers_day4() -> (usize, usize) {
    
    validate_passport()
}

fn validate_passport() -> (usize, usize) {
    const N_REQ_FIELDS: usize = 7;
    let required_fields: [&str; N_REQ_FIELDS] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    // Cheating a bit on the file, stuck when reaching EOF.
    let passports: Vec<HashMap<String, String>> = manage_input::read_passports("inputs/day04_input.txt");
    let mut valid_passports: Vec<HashMap<String, String>> = passports.clone();
    let mut invalid_passports: Vec<HashMap<String, String>> = Vec::new();
    let mut n_valid: usize = passports.len();

    for p in passports {
        for req in required_fields.iter() {
            if p.keys().len() < N_REQ_FIELDS || !p.contains_key(*req) {
                n_valid -= 1;
                invalid_passports.push(p);
                break;
            }
        }
    }

    // Part 2
    valid_passports.retain(|p| !invalid_passports.contains(p));
    let mut n_valid_strict = n_valid;

    for p in valid_passports {
        let keys = p.keys();

        for k in keys {
            let v = p.get(k).unwrap();

            if k == "byr" {
                if v.len() != 4 {
                    n_valid_strict -= 1;
                    break;
                }

                let byr: u16 = v.parse::<u16>().unwrap();

                if byr < 1920 || byr > 2002 {
                    n_valid_strict -= 1;
                    break;
                }
            }
            else if k == "iyr" {
                if v.len() != 4 {
                    n_valid_strict -= 1;
                    break;
                }

                let iyr: u16 = v.parse::<u16>().unwrap();

                if iyr < 2010 || iyr > 2020 {
                    n_valid_strict -= 1;
                    break;
                }
            }
            else if k == "eyr" {
                if v.len() != 4 {
                    n_valid_strict -= 1;
                    break;
                }

                let eyr: u16 = v.parse::<u16>().unwrap();

                if eyr < 2020 || eyr > 2030 {
                    n_valid_strict -= 1;
                    break;
                }
            }
            else if k == "hgt" {
                let hgt_len: usize = v.len();
                let hgt: u16 = v.trim_matches(char::is_alphabetic).parse::<u16>().unwrap();
                let hgt_mag: &str = &v[hgt_len-2..];
                
                if hgt_mag != "in" && hgt_mag != "cm" {
                    // Not checking this was really dumb from my part, it took me so long to realize...
                    n_valid_strict -= 1;
                    break;
                }
                else if hgt_mag == "in" {
                    if hgt < 59 || hgt > 76 {
                        n_valid_strict -= 1;
                        break;
                    }
                }
                else if hgt_mag == "cm" {
                    if hgt < 150 || hgt > 193 {
                        n_valid_strict -= 1;
                        break;
                    }
                }
            }
            else if k == "hcl" {
                // This could be done easier with RegEx.
                let mut hcl: Vec<char> = v.chars().collect();
                let hexadec_chars: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
                                                'a', 'b', 'c', 'd', 'e', 'f'];

                if hcl[0] != '#' || hcl.len() != 7 {
                    n_valid_strict -= 1;
                    break;
                }
                else {
                    hcl.remove(0);

                    for c in hcl {
                        if !hexadec_chars.contains(&c) {
                            n_valid_strict -= 1;
                            break;
                        }
                    }
                }
            }
            else if k == "ecl" {
                let colors: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                let ecl: &str = v;

                if !colors.contains(&ecl) {
                    n_valid_strict -= 1;
                    break;
                }
            }
            else if k == "pid" {
                let pid: Vec<char> = v.chars().collect();

                if pid.len() != 9 {
                    n_valid_strict -= 1;
                    break;
                }

                for d in pid {
                    if !d.is_digit(10) {
                        n_valid_strict -= 1;
                        break;
                    }
                }
            }
        }
    }    
    
    (n_valid, n_valid_strict)
}
