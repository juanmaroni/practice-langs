// Advent of Code 2021
// Day 5: Hydrothermal Venture

use crate::input_handler::parse_vents;

const FILE: &str = "inputs/day05_input.txt";

pub fn day05_answer() {
    let (points, grid_size) = parse_vents(FILE);
    let overlaps = calculate_overlaps(points.clone(), grid_size, false);
    println!("Day 5, part 1: {}", overlaps);

    let overlaps_diagonals = calculate_overlaps(points, grid_size, true);
    println!("Day 5, part 2: {}\n", overlaps_diagonals);
}

fn create_grid(size: usize) -> Vec<Vec<u16>> {
    vec![vec![0; size]; size]
}

fn segment_points(segment_ends: (Vec<u16>, Vec<u16>), count_diagonals: bool) -> Vec<Vec<usize>> {
    let x1 = segment_ends.0[0] as usize;
    let y1 = segment_ends.0[1] as usize;
    let x2 = segment_ends.1[0] as usize;
    let y2 = segment_ends.1[1] as usize;
    let x = vec!(x1, x2);
    let y = vec!(y1, y2);
    let mut segment_points: Vec<Vec<usize>> = Vec::new();

    if x1 == x2 {
        let main_point = x1;

        for i in *y.iter().min().unwrap() ..= *y.iter().max().unwrap() {
            segment_points.push(vec!(main_point, i));
        }
    }
    else if y1 == y2 {
        let main_point = y1;

        for i in *x.iter().min().unwrap() ..= *x.iter().max().unwrap() {
            segment_points.push(vec!(i, main_point));
        }
    }
    else {
        if count_diagonals {
            let x_range: Vec<usize> = if x[0] < x[1] { (x[0] ..= x[1]).collect() } else { (x[1] ..= x[0]).rev().collect() };
            let y_range: Vec<usize> = if y[0] < y[1] { (y[0] ..= y[1]).collect() } else { (y[1] ..= y[0]).rev().collect() };
            let zipper = x_range.iter().zip(y_range.iter());

            for (i, j) in zipper {
                segment_points.push(vec!(*i, *j));
            }
        }
    }

    segment_points
}

fn calculate_overlaps(points: Vec<(Vec<u16>, Vec<u16>)>, grid_size: u16, count_diagonals: bool) -> u16 {
    let mut grid = create_grid(grid_size as usize);

    for p in points {
        let segment_points = segment_points(p, count_diagonals);

        for sp in segment_points {
            grid[sp[0]][sp[1]] += 1;
        }
    }

    grid.iter().flat_map(|r| r.iter()).filter(|n| **n > 1).count() as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day05_input_test.txt";

    #[test]
    fn day05_part1_test() {
        let (points, grid_size) = parse_vents(FILE);
        let overlaps = calculate_overlaps(points, grid_size, false);

        assert_eq!(overlaps, 5);
    }

    #[test]
    fn day05_part2_test() {
        let (points, grid_size) = parse_vents(FILE);
        let overlaps = calculate_overlaps(points, grid_size, true);

        assert_eq!(overlaps, 12);
    }
}
