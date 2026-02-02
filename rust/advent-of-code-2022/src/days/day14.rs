// Advent of Code 2022
// Day 14: Regolith Reservoir

use std::{io::BufRead, collections::HashSet};
use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day14_input.txt";

pub fn print_answers() {
    let mut day = Day::new(14, FILE.to_string());

    let rock_points = get_rock_points(parse_input(&day));

    day.first_answer = Some(Answer::Num(0));
    day.second_answer = Some(Answer::Num(0));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> Vec<Vec<Vec<u16>>> {
    day.read_file()
        .lines()
        .map(|line| line.unwrap()
            .split(" -> ")
            .map(|points| points.split(',')
                .map(|p| p.parse::<u16>().unwrap())
                .collect::<Vec<u16>>())
            .collect::<Vec<Vec<u16>>>())
        .collect()
}

// Helping function for parsing
fn get_rock_points(rockpaths: Vec<Vec<Vec<u16>>>) -> HashSet<(u16, u16)> {
    let mut rock_points: HashSet<(u16, u16)> = HashSet::new();
    
    for i in rockpaths {
        for rp in i.windows(2) {
            let px1 = rp[0][0];
            let py1 = rp[0][1];
            let px2 = rp[1][0];
            let py2 = rp[1][1];
    
            if px1 == px2 {
                if py1 > py2 {
                    rock_points.extend((py2..=py1).map(|y| (px1, y)));
                } else {
                    rock_points.extend((py1..=py2).map(|y| (px1, y)));
                }
            } else if py1 == py2 {
                if px1 > px2 {
                    rock_points.extend((px2..=px1).map(|x| (x, py1)));
                } else {
                    rock_points.extend((px1..=px2).map(|x| (x, py1)));
                }
            }
        }
    }

    rock_points
}

// Part 1
fn count_units_sand(rock_points: &HashSet<(u16, u16)>) -> u64 {
    

    todo!()
}

// Part 2
fn calc_second_answer(rock_points: &HashSet<(u16, u16)>) -> u64 {
    todo!()
}

// Helping function for Part 1
fn simulate_sand_movement(rock_points: &HashSet<(u16, u16)>) {
    let mut rock_points = rock_points.clone();
    //let mut sand_points: HashSet<(u16, u16)> = HashSet::new();
    let mut start: (u16, u16) = (500, 0);
    let mut count_sand = 0;
    
    loop {
        // Search the boundaries
        let deepest_y = *rock_points.iter().map(|(_, y)| y).max().unwrap();
        let deepest_x = *rock_points.iter().map(|(x, _)| x).max().unwrap();


        let y = deepest_y - 1;

        break;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day14_input_test.txt";

    #[test]
    fn day14_part1_test() {
        let mut day = Day::new(14, FILE.to_string());
        let rock_points = get_rock_points(parse_input(&day));
        simulate_sand_movement(&rock_points);

        assert_eq!(7, 7);

        day.first_answer = Some(Answer::Num(0));
    }

    #[test]
    fn day14_part2_test() {
        let mut day = Day::new(14, FILE.to_string());
        

        assert_eq!(5, 5);

        day.second_answer = Some(Answer::Num(0));
    }
}
