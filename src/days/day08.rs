// Advent of Code 2022
// Day 8: Treetop Tree House

use std::io::BufRead;

use aoc2022::{Day, Part, Answer};

const FILE: &str = "inputs/real/day08_input.txt";

pub fn print_answers() {
    let mut day = Day::new(8, FILE.to_string());

    let tree_grid = parse_input(&day);

    day.first_answer = Some(Answer::Num(count_visible_trees(&tree_grid) as u64));
    day.second_answer = Some(Answer::Num(0));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn parse_input(day: &Day) -> Vec<Vec<u8>> {
    day.read_file()
        .lines()
        .map(|line| line
                .unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>()
}

// Part 1
fn count_visible_trees(tree_grid: &Vec<Vec<u8>>) -> u16 {
    let grid_len_x = tree_grid.len() - 1;
    let grid_len_y = tree_grid[0].len() - 1;
    let mut count = (grid_len_x * 2 + grid_len_y * 2) as u16;

    for x in 1..grid_len_x {
        for y in 1..grid_len_y {
            if check_viewable(x, grid_len_x, y, grid_len_y, tree_grid) {
                count += 1;   
            }
        }
    }
    
    count
}

// Part 2
fn calc_second_answer(tree_grid: &Vec<Vec<u8>>) -> u16 {
    todo!()
}

// Helping function for Part 1
fn check_viewable(x: usize, grid_len_x: usize, y: usize, grid_len_y: usize, tree_grid: &Vec<Vec<u8>>) -> bool {
    let elem = tree_grid[x][y];
    let mut non_viewable: u8 = 0;

    // Left
    for xs in 0..x {
        if elem <= tree_grid[xs][y] {
            non_viewable += 1;
            break;
        }
    }

    // Right
    for xs in x+1..=grid_len_x {
        if elem <= tree_grid[xs][y] {
            non_viewable += 1;
            break;
        }
    }

    // Up
    for ys in 0..y {
        if elem <= tree_grid[x][ys] {
            non_viewable += 1;
            break;
        }
    }

    // Down
    for ys in y+1..=grid_len_y {
        if elem <= tree_grid[x][ys] {
            non_viewable += 1;
            break;
        }
    }

    non_viewable < 4
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day08_input_test.txt";

    #[test]
    fn day08_part1_test() {
        let mut day = Day::new(8, FILE.to_string());
        let tree_grid = parse_input(&day);
        let ans = count_visible_trees(&tree_grid);

        assert_eq!(ans, 21);

        day.first_answer = Some(Answer::Num(ans as u64));
    }

    #[test]
    fn day08_part2_test() {
        let mut day = Day::new(8, FILE.to_string());
        

        assert_eq!(5, 5);

        day.second_answer = Some(Answer::Num(0));
    }
}
