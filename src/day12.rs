// Advent of Code 2020: December, 12
// Day 12: Rain Risk

use core::panic;
use crate::manage_input::get_ship_instructions;

pub fn answers_day12() -> (i16, i16) {
    let filepath: &str = "inputs/day12_input.txt";
    let instructions = get_ship_instructions(filepath);
    let manhattan_dis = manhattan_distance(move_ship(instructions.clone()));
    let manhattan_dis_waypoint = manhattan_distance(move_with_waypoint(instructions));

    (manhattan_dis, manhattan_dis_waypoint)
}

fn manhattan_distance(location: (i16, i16)) -> i16 {
    location.0.abs() + location.1.abs()
}

fn move_ship(instructions: Vec<(char, i16)>) -> (i16, i16) {
    // (+E -W, +N -S)
    let mut location: (i16, i16) = (0, 0);
    // Numeric system 0-3, ENWS
    // Dividing by 90º and applying modulus 4, E-0, S-1, W-2, N-3.
    let mut direction: i16 = 0;

    for mut m in instructions {
        if m.0 == 'F' {
            match direction.abs() {
                0 => m.0 = 'E',
                1 => m.0 = 'N',
                2 => m.0 = 'W',
                3 => m.0 = 'S',
                _ => panic!("The value of direction is: {}", direction)
            };
        }

        match m.0 {
            'N' => location.1 += m.1,
            'S' => location.1 -= m.1,
            'E' => location.0 += m.1,
            'W' => location.0 -= m.1,
            'L' => direction = (direction + m.1 / 90) % 4,
            'R' => direction = (direction + 4 - m.1 / 90) % 4,
            _ => panic!()
        };
    }

    location
}

fn move_with_waypoint(instructions: Vec<(char, i16)>) -> (i16, i16) {
    // (+E -W, +N -S)
    let mut location: (i16, i16) = (0, 0);
    let mut waypoint: (i16, i16) = (10, 1);

    for m in instructions {
        if m.0 == 'F' {
            location.1 += waypoint.1 * m.1;
            location.0 += waypoint.0 * m.1;
        }
        else {
            match m.0 {
                'N' => waypoint.1 += m.1,
                'S' => waypoint.1 -= m.1,
                'E' => waypoint.0 += m.1,
                'W' => waypoint.0 -= m.1,
                'L' => waypoint = rotate(waypoint, 360-m.1),
                'R' => waypoint = rotate(waypoint, m.1),
                _ => panic!()
            };
        }
    }

    location
}

fn rotate(point: (i16, i16), degrees: i16) -> (i16, i16) {
    match degrees {
        0 => (point.0, point.1),
        90 => (point.1, -point.0),
        180 => (-point.0, -point.1),
        270 => (-point.1, point.0),
        _ => panic!()
    }
}
