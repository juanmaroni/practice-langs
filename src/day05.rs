// Advent of Code 2020: December, 5
// Day 5: Binary Boarding

use crate::manage_input::parse_lines;

pub fn answers_day5() -> (u32, u32) {
    info_seat_id()
}

fn get_row(code: &str) -> u8 {
    let code_chars: Vec<char> = code.chars().collect();
    let mut exp: u32 = 7;
    let mut row: u8 = 0;

    for c in code_chars {
        exp -= 1;

        if c == 'B' {
            row += u8::pow(2, exp);
        }
    }

    row
}

fn get_col(code: &str) -> u8 {
    let code_chars: Vec<char> = code.chars().collect();
    let mut exp: u32 = 3;
    let mut col: u8 = 0;

    for c in code_chars {
        exp -= 1;

        if c == 'R' {
            col += u8::pow(2, exp);
        }        
    }

    col
}

fn get_seat_id(row: u8, col: u8) -> u32 {
    u32::from(row) * 8 + u32::from(col)
}

fn info_seat_id() -> (u32, u32) {
    let passes = parse_lines("inputs/day05_input.txt");
    let mut seat_ids: Vec<u32> = Vec::new();
    let mut max_id = 0;

    for p in passes {
        let row = get_row(&p[..7]);
        let col = get_col(&p[7..]);
        let seat_id = get_seat_id(row, col);

        if seat_id > max_id {
            max_id = seat_id;
        }

        seat_ids.push(seat_id);
    }

    // Part 2
    seat_ids.sort();
    let mut my_seat = 0;

    for id in &seat_ids {
        let next = id + 1;

        if !seat_ids.contains(&next) && seat_ids.contains(&(id + 2)) {
            my_seat = next;
        }
    }

    (max_id, my_seat)
}
