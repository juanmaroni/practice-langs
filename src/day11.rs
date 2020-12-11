// Advent of Code 2020: December, 11
// Day 11: Seating System

use crate::manage_input;

pub fn answers_day11() -> (u16, u64) {
    let filepath: &str = "inputs/day11_input.txt";
    (total_occupied_seats(manage_input::generate_matrix(filepath)), 0)
}

// Part 1
fn total_occupied_seats(seats: Vec<Vec<char>>) -> u16 {
    let mut current_seats = seats;
    let row_len = current_seats.len();
    let col_len = current_seats[0].len();

    loop {
        let mut update_seats: Vec<Vec<char>> = Vec::new();
        let mut count_occupied: u16 = 0;

        for i in 1..row_len - 1 {
            let mut update_row: Vec<char> = Vec::new();
            update_row.push('_');

            for j in 1..col_len - 1 {
                // Get adjacent neighbours
                let adjacents_occupied_seats = check_adjacents_seats(i, j, &current_seats);
                
                // Counting occupied.
                if current_seats[i][j] == '#' {
                    count_occupied += 1;
                }

                if current_seats[i][j] == 'L' && adjacents_occupied_seats == 0 {
                    update_row.push('#');
                }
                else if current_seats[i][j] == '#' && adjacents_occupied_seats >= 4 {
                    update_row.push('L');
                }
                else {
                    update_row.push(current_seats[i][j]);
                }
            }

            update_row.push('_');
            update_seats.push(update_row);
        }

        let mut m_blank: Vec<char> = Vec::new();

        for _ in 0..row_len {
            m_blank.push('_');
        }

        update_seats.insert(0, m_blank.clone());
        update_seats.push(m_blank);
        
        if current_seats != update_seats {
            current_seats = update_seats;
        }
        else {
            return count_occupied;
        }
    }
}

fn check_adjacents_seats(i: usize, j: usize, seats: &Vec<Vec<char>>) -> u8 {
    let mut occupied: u8 = 0;

    if seats[i-1][j] == '#' {
        occupied += 1;
    }
    if seats[i][j-1]== '#' {
        occupied += 1;
    }
    if seats[i-1][j-1] == '#' {
        occupied += 1;
    }
    if seats[i+1][j] == '#' {
        occupied += 1;
    }
    if seats[i][j+1] == '#' {
        occupied += 1;
    }
    if seats[i+1][j+1] == '#' {
        occupied += 1;
    }
    if seats[i+1][j-1] == '#' {
        occupied += 1;
    }
    if seats[i-1][j+1] == '#' {
        occupied += 1;
    }

    occupied
}
