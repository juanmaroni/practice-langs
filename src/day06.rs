// Advent of Code 2020: December, 6
// Day 6: Custom Customs

use crate::manage_input;

pub fn answers_day6() -> (u16, u16) {
    count_yes()
}

fn count_yes() -> (u16, u16) {
    let answers_by_people = manage_input::read_answers("inputs/day06_input.txt");
    let mut any_yes: u16 = 0;
    let all_yes: u16 = answers_by_people.1;

    for a in answers_by_people.0 {
        any_yes += a.1.len() as u16;
    }

    (any_yes, all_yes)
}


