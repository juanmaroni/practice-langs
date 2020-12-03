// Advent of Code 2020: December, 3
// Day 3: Toboggan Trajectory

use crate::manage_input;

pub fn answers_day3() -> (usize, usize) {
    let part1: usize = trees_found((3, 1));

    // I had a problem here trying to play with different integer data types
    let part2: usize = trees_found((1, 1)) * trees_found((3, 1)) * trees_found((5, 1)) * trees_found((7, 1)) * trees_found((1, 2));
    
    (part1, part2)
}

fn trees_found(slope: (usize, usize)) -> usize {
    let input_info: (String, usize) = manage_input::grid_from_file("inputs/day03_input.txt");
    let grid: String = input_info.0;
    let len_pattern = input_info.1;
    let mut n_trees: usize = 0;
    let mut hpos = 0;

    // I needed to look for help, but I think that I learned something new... I hope so.
    for p in grid.split('\n').step_by(slope.1).skip(1) {
        hpos = (hpos + slope.0) % len_pattern;
        
        if p.chars().nth(hpos) == Some('#') {
            n_trees += 1;
        }
    }

    n_trees
}
