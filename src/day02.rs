// Advent of Code 2020: December, 2
// Day 2: Password Philosophy

use crate::manage_input;

pub fn answers_day2() -> (u16, u16) {
    let valid_pw: (Vec<PasswordPolicy>, u16, u16) = check_password_policy();

    (valid_pw.1, valid_pw.2)
}

// The struct is not necessary, I just wanted to play with it.
struct PasswordPolicy {
    min: u8,
    max: u8,
    letter: char,
    password: String,
}

impl PasswordPolicy {
    fn valid_password_repeat(&self) -> bool {
        let char_reps: u8 = self.password.matches(self.letter).count() as u8;
    
        if char_reps >= self.min && char_reps <= self.max {
            return true;
        }
        
        false
    }

    fn valid_password_position(&self) -> bool {
        // The system index starts at 1, so I will convert to 0
        let index_min: usize = usize::from(self.min - 1);
        let index_max: usize = usize::from(self.max - 1);
    
        if (self.password.chars().nth(index_min).unwrap() == self.letter
            && self.password.chars().nth(index_max).unwrap() != self.letter)
            || (self.password.chars().nth(index_min).unwrap() != self.letter
            && self.password.chars().nth(index_max).unwrap() == self.letter) {
            return true;
        }
        
        false
    }
}

fn check_password_policy() -> (Vec<PasswordPolicy>, u16, u16) {
    let file_passwords: Vec<String> = manage_input::passwords_from_file("inputs/day02_input.txt");
    // I don't need to return a vector, just wanted to do it for learning purposes
    let mut pw_policies: Vec<PasswordPolicy> = Vec::new();
    let mut invalid_pw_repeat: u16 = 0;
    let mut invalid_pw_position: u16 = 0;

    for pw in file_passwords {
        // This will return a vector of five elements, one of them (index 3, "") is useless,
        // so I will skip it when assigning values to the struct.
        let pass_policy_params: Vec<_> = pw.split(|c: char| !c.is_alphanumeric()).collect();
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

        pw_policies.push(pass_policy);
    }

    (pw_policies, invalid_pw_repeat, invalid_pw_position)
}
