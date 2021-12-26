// Advent of Code 2021
// Day 9: Smoke Basin

use crate::input_handler::parse_heightmap;

const FILE: &str = "inputs/day09_input.txt";

pub fn day09_answer() {
    let mut heightmap = parse_heightmap(FILE);
    println!("Day 9, part 1: {}", count_risk_levels(&heightmap));
    
    let (b1, b2, b3) = find_three_largest_basins(&mut heightmap);
    println!("Day 9, part 2: {}\n", b1 as usize * b2 as usize * b3 as usize);
}

// Up, right, left, down
fn find_elements_with_greater_adjacents(row: isize, col: isize, element: i8, matrix: &Vec<Vec<i8>>, n_rows: isize, n_cols: isize) -> i8 {
    let prev_row = row - 1;
    let prev_col = col - 1;
    let next_row = row + 1;
    let next_col = col + 1;
    
    // Outside top-left corner
    if prev_row < 0 && prev_col < 0 {
        if (element < matrix[next_row as usize][col as usize]) 
            && (element < matrix[row as usize][next_col as usize]) {
            return element + 1;
        }
    }
    // Outside top-right corner
    else if prev_row < 0 && next_col > n_cols {
        if (element < matrix[next_row as usize][col as usize]) 
            && (element < matrix[row as usize][prev_col as usize]) {
            return element + 1;
        }
    }
    // Outside bottom-right corner
    else if next_row > n_rows && next_col > n_cols {
        if (element < matrix[prev_row as usize][col as usize]) 
            && (element < matrix[row as usize][prev_col as usize]) {
            return element + 1;
        }
    }
    // Outside bottom-left corner
    else if next_row > n_rows && prev_col < 0 {
        if (element < matrix[prev_row as usize][col as usize]) 
            && (element < matrix[row as usize][next_col as usize]) {
            return element + 1;
        }
    }
    // Outside top
    else if prev_row < 0 && prev_col >= 0 {
        if (element < matrix[next_row as usize][col as usize]) 
            && (element < matrix[row as usize][next_col as usize])
            && (element < matrix[row as usize][prev_col as usize]) {
            return element + 1;
        }
    }
    // Outside right
    else if next_row <= n_rows && prev_col < 0 {
        if (element < matrix[next_row as usize][col as usize]) 
            && (element < matrix[prev_row as usize][col as usize]) 
            && (element < matrix[row as usize][next_col as usize]) {
            return element + 1;
        }
    }
    // Outside left
    else if prev_row >= 0 && next_col > n_cols {
        if (element < matrix[next_row as usize][col as usize]) 
            && (element < matrix[prev_row as usize][col as usize]) 
            && (element < matrix[row as usize][prev_col as usize]) {
            return element + 1;
        }
    }
    // Outside bottom
    else if next_row > n_rows && prev_col <= n_cols {
        if (element < matrix[row as usize][next_col as usize])
            && (element < matrix[prev_row as usize][col as usize]) 
            && (element < matrix[row as usize][prev_col as usize]) {
            return element + 1;
        }
    }
    else {
        if (element < matrix[next_row as usize][col as usize]) 
            && (element < matrix[row as usize][next_col as usize])
            && (element < matrix[prev_row as usize][col as usize]) 
            && (element < matrix[row as usize][prev_col as usize]) {
            return element + 1;
        }
    }

    0
}

fn count_risk_levels(heightmap: &Vec<Vec<i8>>) -> usize {
    let mut count = 0;
    let max_index_rows = heightmap.len() - 1;
    let max_index_cols = heightmap[0].len() - 1;

    for i in 0..=max_index_rows {
        for j in 0..=max_index_cols {
            let elem = heightmap[i][j];
            
            if elem != 9 {
                count += find_elements_with_greater_adjacents(i as isize, j as isize, elem, heightmap, max_index_rows as isize, max_index_cols as isize) as usize;
            }
        }
    }
    
    count
}

fn add_padding(heightmap: &mut Vec<Vec<i8>>, size: usize, n: i8) {
    for r in heightmap.iter_mut() {
        for _ in 0..size {
            r.insert(0, n);
            r.push(n);
        }
    }

    let size_line = heightmap[0].len();

    for _ in 0..size {
        heightmap.insert(0, (0..size_line).map(|_| n).collect::<Vec<i8>>());
        heightmap.push((0..size_line).map(|_| n).collect::<Vec<i8>>());
    }
}

// Find non basin locations (9's), count them and replace them with 9's recursively
fn fill_heightmap(heightmap: &mut Vec<Vec<i8>>, row: usize, col: usize, mut count: u16) -> u16 {
    if heightmap[row][col] != 9 {
        count += 1;
        heightmap[row][col] = 9;
        count = fill_heightmap(heightmap, row, col + 1, count);
        count = fill_heightmap(heightmap, row, col - 1, count);
        count = fill_heightmap(heightmap, row + 1, col, count);
        count = fill_heightmap(heightmap, row - 1, col, count);
    }
    
    count
}

fn find_three_largest_basins(heightmap: &mut Vec<Vec<i8>>) -> (u16, u16, u16) {
    // Fill borders with 9's
    add_padding(heightmap, 1, 9);
    
    let mut basins: Vec<u16> = Vec::new();
    let max_index_rows = heightmap.len();
    let max_index_cols = heightmap[0].len();

    for row in 1..max_index_rows - 1 {
        for col in 1..max_index_cols - 1 {
            let basin = fill_heightmap(heightmap, row, col, 0);

            if basin != 0 {
                basins.push(basin);
            }
        }
    }

    basins.sort();
    
    (basins.pop().unwrap(), basins.pop().unwrap(), basins.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day09_input_test.txt";

    #[test]
    fn day09_part1_test() {
        let heightmap = parse_heightmap(FILE);
        
        assert_eq!(count_risk_levels(&heightmap), 15);
    }

    #[test]
    fn day09_part2_test() {
        let mut heightmap = parse_heightmap(FILE);
        let (b1, b2, b3) = find_three_largest_basins(&mut heightmap);

        assert_eq!(b1 * b2 * b3, 1134);
    }
}
