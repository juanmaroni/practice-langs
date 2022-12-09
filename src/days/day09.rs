// Advent of Code 2022
// Day 9: Rope Bridge

use std::io::BufRead;
use std::collections::HashSet;
use itertools::Itertools;
use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day09_input.txt";

pub fn print_answers() {
    let mut day = Day::new(9, FILE.to_string());

    let motions = parse_input(&day);
    get_unique_positions(&motions);

    day.first_answer = Some(Answer::Num(0));
    day.second_answer = Some(Answer::Num(0));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> Vec<(char, u8)> {
    let mut motions: Vec<(char, u8)> = Vec::new();
    
    for line in day.read_file().lines() {
        let content = line.unwrap();
        let mut split = content.split_whitespace();

        motions.push((split.next().unwrap().chars().nth(0).unwrap(),
                      split.next().unwrap().parse::<u8>().unwrap()));
    }

    motions
}

// Part 1
fn count_unique_positions_tail(motions: &Vec<(char, u8)>) -> u32 {

    todo!()
}

// Part 2
fn calc_second_answer(motions: &Vec<(char, u8)>) -> u32 {
    todo!()
}

// Helping function for Part 1
fn get_unique_positions(motions: &Vec<(char, u8)>) -> HashSet<(i16, i16)>{
    // Positions as tuples (x, y).
    let mut unique_positions_tail: HashSet<(i16, i16)> = HashSet::new();

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    unique_positions_tail.insert(tail_pos);

    for (d, s) in motions {
        for _ in 0..*s {
            (head_pos, tail_pos) = head_motion(*d, head_pos, tail_pos);

            unique_positions_tail.insert(tail_pos);
            println!("{:?}", tail_pos);
        }        
    }
    
    unique_positions_tail
}

fn head_motion(dir: char, mut head_pos: (i16, i16), tail_pos: (i16, i16)) -> ((i16, i16), (i16, i16)) {
    if dir == 'U' {
        head_pos.1 += 1;
    } else if dir == 'D' {
        head_pos.1 -= 1;
    } else if dir == 'R' {
        head_pos.0 += 1;
    } else if dir == 'L' {
        head_pos.0 -= 1;
    }

    (head_pos, tail_motion(head_pos, tail_pos))
}

fn tail_motion(head_pos: (i16, i16), mut tail_pos: (i16, i16)) -> (i16, i16) {
    let (head_x, head_y) = head_pos;
    let (tail_x, tail_y) = tail_pos;

    let dis_x = head_x.abs() - tail_x.abs();
    let dis_y = head_y.abs() - tail_y.abs();
    
    if dis_x == 2 && dis_y == 0 { // X-axis
        if head_x > tail_x {
            tail_pos.0 += 1;
        } else if head_x < tail_x {
            tail_pos.0 -= 1;
        }
    } else if dis_y == 2 && dis_x == 0 { // Y-axis
        if head_y > tail_y {
            tail_pos.1 += 1;
        } else if head_y < tail_y {
            tail_pos.1 -= 1;
        }
    } else if dis_x == 2 && dis_y == 1 { // Diagonally THERE ARE FOUR
        if head_x > tail_x {
            tail_pos.0 += 1;
        } else if head_x < tail_x {
            tail_pos.0 -= 1;
        }

        if head_y > tail_y {
            tail_pos.1 += 1;
        } else if head_y < tail_y {
            tail_pos.1 -= 1;
        }
    } else if dis_x == 1 && dis_y == 2 { // Diagonally
        if head_y > tail_y {
            tail_pos.1 += 1;
        } else if head_y < tail_y {
            tail_pos.1 -= 1;
        }

        if head_x > tail_x {
            tail_pos.0 += 1;
        } else if head_x < tail_x {
            tail_pos.0 -= 1;
        }
    }

    tail_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day09_input_test.txt";

    #[test]
    fn day09_part1_test() {
        let mut day = Day::new(9, FILE.to_string());
        let motions = parse_input(&day);
        println!("{:?}", motions);
        get_unique_positions(&motions);

        assert_eq!(13, 13);

        day.first_answer = Some(Answer::Num(0));
    }

    #[test]
    fn day09_part2_test() {
        let mut day = Day::new(9, FILE.to_string());
        let motions = parse_input(&day);

        assert_eq!(5, 5);

        day.second_answer = Some(Answer::Num(0));
    }
}
