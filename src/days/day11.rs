// Advent of Code 2021
// Day 11: Dumbo Octopus

use crate::input_handler::parse_octopus_energy_levels;

const FILE: &str = "inputs/day11_input.txt";

pub fn day11_answer() {
    let mut octopus_energy_levels = parse_octopus_energy_levels(FILE);

    println!("Day 11, part 1: {}", count_flashes(&mut octopus_energy_levels, 100).0);

    octopus_energy_levels = parse_octopus_energy_levels(FILE);

    println!("Day 11, part 2: {}\n", count_flashes(&mut octopus_energy_levels, 9999).1);
}

fn increase_energy_around(grid: &mut Vec<Vec<i8>>, row: usize, col: usize, n_rows: usize, n_cols: usize) {
    let prev_row = row as isize - 1;
    let prev_col = col as isize - 1;
    let next_row = row as isize + 1;
    let next_col = col as isize + 1;
    grid[row][col] = -1;

    // Top
    if row > 0 {
        // Top-Left
        if col > 0 && grid[prev_row as usize][prev_col as usize] >= 0 {
            grid[prev_row as usize][prev_col as usize] += 1;
                
            if grid[prev_row as usize][prev_col as usize] > 9 {
                increase_energy_around(grid, prev_row  as usize, prev_col as usize, n_rows, n_cols);
            }
        }

        // Top-Center
        if grid[prev_row as usize][col] >= 0 {
            grid[prev_row as usize][col] += 1;

            if grid[prev_row as usize][col] > 9 {
                increase_energy_around(grid, prev_row  as usize, col, n_rows, n_cols);
            }
        }

        // Top-Right
        if col < n_cols && grid[prev_row as usize][next_col as usize] >= 0 {
            grid[prev_row as usize][next_col as usize] += 1;

            if grid[prev_row as usize][next_col as usize] > 9 {
                increase_energy_around(grid, prev_row as usize, next_col as usize, n_rows, n_cols);
            }
        }
    }

    // Left
    if col > 0 && grid[row][prev_col as usize] >= 0 {
        grid[row][prev_col as usize] += 1;

        if grid[row][prev_col as usize] > 9 {
            increase_energy_around(grid, row, prev_col as usize, n_rows, n_cols);
        }
    }

    // Right
    if col < n_cols && grid[row][next_col as usize] >= 0 {
        grid[row][next_col as usize] += 1;

        if grid[row][next_col as usize] > 9 {
            increase_energy_around(grid, row, next_col as usize, n_rows, n_cols);
        }
    }

    // Bottom
    if row < n_rows {
        // Bottom-Left
        if col > 0 && grid[next_row as usize][prev_col as usize] >= 0 {
            grid[next_row as usize][prev_col as usize] += 1;

            if grid[next_row as usize][prev_col as usize] > 9 {
                increase_energy_around(grid, next_row as usize, prev_col as usize, n_rows, n_cols);
            }
        }

        // Bottom-Center
        if grid[next_row as usize][col] >= 0 {
            grid[next_row as usize][col] += 1;

            if grid[next_row as usize][col] > 9 {
                increase_energy_around(grid, next_row as usize, col, n_rows, n_cols);
            }
        }

        // Bottom-Right
        if col < n_cols && grid[next_row as usize][next_col as usize] >= 0 {
            grid[next_row as usize][next_col as usize] += 1;

            if grid[next_row as usize][next_col as usize] > 9 {
                increase_energy_around(grid, next_row as usize, next_col as usize, n_rows, n_cols);
            }
        }
    }
}

fn count_flashes(energy_levels: &mut Vec<Vec<i8>>, steps: usize) -> (usize, usize) {
    let mut n_flashes = 0;
    let mut sync_step = 0;
    let max_index_rows = energy_levels.len() - 1;
    let max_index_cols = energy_levels[0].len() - 1;

    for s in 0..steps {        
        for i in 0..=max_index_rows {
            for j in 0..=max_index_cols {
                if energy_levels[i][j] >= 0 {
                    energy_levels[i][j] += 1;
                
                    if energy_levels[i][j] > 9 {
                        increase_energy_around(energy_levels, i, j, max_index_rows, max_index_cols);
                    }
                }
            }
        }

        for row in energy_levels.iter_mut() {
            for cell in row.iter_mut() {
                if *cell < 0 {
                    n_flashes += 1;
                    *cell = 0;
                }
            }
        }

        if energy_levels.iter().flat_map(|n| n.iter().map(|n| *n as u16)).sum::<u16>() == 0 {
            sync_step = s + 1;
            break;
        }
    }

    (n_flashes, sync_step)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day11_input_test.txt";

    #[test]
    fn day11_part1_test() {
        let mut octopus_energy_levels = parse_octopus_energy_levels(FILE);

        assert_eq!(count_flashes(&mut octopus_energy_levels, 100).0, 1656);
    }

    #[test]
    fn day11_part2_test() {
        let mut octopus_energy_levels = parse_octopus_energy_levels(FILE);

        assert_eq!(count_flashes(&mut octopus_energy_levels, 200).1, 195);
    }
}
