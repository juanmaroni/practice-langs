// Advent of Code 2021
// Day 13: Transparent Origami

use crate::input_handler::parse_manual_instructions;

const FILE: &str = "inputs/day13_input.txt";

pub fn day13_answer() {
    let (points, folds) = parse_manual_instructions(FILE);
    let mut grid = build_grid(&points);
    
    println!("Day 13, part 1: {}", fold_page_once(&mut grid, folds[0]));
    println!("Day 13, part 2: \n");
    fold_page(&mut grid, folds)
}

fn build_grid(points: &Vec<(usize, usize)>) -> Vec<Vec<bool>> {
    let grid_size_x= points.iter().map(|(x, _)| *x).max().unwrap() * 2 + 1;
    let grid_size_y= points.iter().map(|(_, y)| *y).max().unwrap() * 2 + 1;
    let mut grid = vec![vec![false; grid_size_x]; grid_size_y];

    for (x, y) in points {
        grid[*y][*x] = true;
    }

    grid
}

fn fold_page_once(grid: &mut Vec<Vec<bool>>, fold: (usize, usize)) -> usize {
    // Horizontal line
    if fold.0 == 0 {
        let fold_line = fold.1;
        let mut down_line = fold_line + 1;
        let range = {
            if fold_line > grid.len() - fold_line - 1 {
                (grid.len() - fold_line - 1)..fold_line
            }
            else {
                0..fold_line
            }
        };

        for x in range.rev() {
            for y in 0..grid[0].len() {
                grid[x][y] = grid[x][y] || grid[down_line][y];
            }

            down_line += 1;
        }

        grid.drain(fold_line..);
    }
    // Vertical line
    else if fold.1 == 0 {
        let fold_line = fold.0;

        for x in 0..grid.len() {
            let mut right_line = fold_line + 1;
            let range = {
                if fold_line > grid[0].len() - fold_line - 1 {
                    (grid[0].len() - fold_line - 1)..fold_line
                }
                else {
                    0..fold_line
                }
            };
            
            for y in range.rev() {
                grid[x][y] = grid[x][y] || grid[x][right_line];
                
                right_line += 1;
            }
        }

        for v in grid.iter_mut() {
            v.drain(fold_line..);
        }
    }

    grid.iter().flat_map(|row| row.iter().filter(|e| **e == true)).count()
}

fn fold_page(grid: &mut Vec<Vec<bool>>, folds: Vec<(usize, usize)>) {
    for f in folds[1..].iter() {
        fold_page_once(grid, *f);
    }

    for row in grid {
        for col in row {
            if *col {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day13_input_test.txt";

    #[test]
    fn day13_part1_test() {
        let (points, folds) = parse_manual_instructions(FILE);
        let mut grid = build_grid(&points);
        
        assert_eq!(fold_page_once(&mut grid, folds[0]), 17);
        assert_eq!(fold_page_once(&mut grid, folds[1]), 16);
    }

    #[test]
    fn day13_part2_test() {
        let (points, folds) = parse_manual_instructions(FILE);
        let mut grid = build_grid(&points);

        fold_page_once(&mut grid, folds[0]);
        fold_page(&mut grid, folds);
    }
}
