// Advent of Code 2020: December, 3
// Day 3: Toboggan Trajectory

use crate::manage_input::parse_grid;

pub fn answers_day3() -> (usize, usize) {
    let part1 = trees_found((3, 1));

    let part2 = trees_found((1, 1))
        * part1
        * trees_found((5, 1))
        * trees_found((7, 1))
        * trees_found((1, 2));
    
    (part1, part2)
}

fn trees_found(slope: (usize, usize)) -> usize {
    let grid = parse_grid("inputs/day03_input.txt");
    let len_pattern = grid.find('\n').unwrap();
    let mut n_trees = 0;
    let mut hpos = 0;

    for p in grid.split('\n').step_by(slope.1).skip(1) {
        hpos = (hpos + slope.0) % len_pattern;
        
        if p.chars().nth(hpos) == Some('#') {
            n_trees += 1;
        }
    }

    n_trees
}
