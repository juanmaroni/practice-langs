// Advent of Code 2020: December, 2
// Day 2: Password Philosophy

use crate::manage_input::parse_lines;

pub fn answers_day2() -> (u16, u16) {
    let valid_pw: (u16, u16) = check_password_policy();

    (valid_pw.0, valid_pw.1)
}

struct PasswordPolicy {
    min: u8,
    max: u8,
    letter: char,
    password: String,
}

impl PasswordPolicy {
    fn valid_password_repeat(&self) -> bool {
        let char_reps: u8 = self.password.matches(self.letter).count() as u8;
    
        char_reps >= self.min && char_reps <= self.max
    }

    fn valid_password_position(&self) -> bool {
        // The system index starts at 1, so I will convert to 0
        let index_min: usize = usize::from(self.min - 1);
        let index_max: usize = usize::from(self.max - 1);

        let char_first_pos: char = self.password.chars().nth(index_min).unwrap();
        let char_last_pos: char = self.password.chars().nth(index_max).unwrap();
    
        (char_first_pos == self.letter && char_last_pos != self.letter)
        || (char_first_pos != self.letter && char_last_pos == self.letter)
    }
}

fn check_password_policy() -> (u16, u16) {
    let passwords: Vec<String> = parse_lines("inputs/day02_input.txt");
    let mut invalid_pw_repeat: u16 = 0;
    let mut invalid_pw_position: u16 = 0;

    for pw in passwords {
        // This will return a vector of five elements, one of them (index 3, "") is useless,
        // so I will skip it when assigning values to the struct.
        let pass_policy_params: Vec<&str> = pw.split(|c: char| !c.is_alphanumeric()).collect();
        let pass_policy = PasswordPolicy {
            min: pass_policy_params[0].parse::<u8>().unwrap(),
            max: pass_policy_params[1].parse::<u8>().unwrap(),
            letter: pass_policy_params[2].chars().nth(0).unwrap(),
            password: String::from(pass_policy_params[4]),
        };

        if pass_policy.valid_password_repeat() {
            invalid_pw_repeat += 1;
        }

        if pass_policy.valid_password_position() {
            invalid_pw_position += 1;
        }
    }

    (invalid_pw_repeat, invalid_pw_position)
}
