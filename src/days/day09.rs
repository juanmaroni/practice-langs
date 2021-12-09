// Advent of Code 2021
// Day 9: Smoke Basin

use crate::input_handler::parse_heightmap;

const FILE: &str = "inputs/day09_input.txt";

pub fn day09_answer() {
    let heightmap = parse_heightmap(FILE);

    println!("Day 9, part 1: {}", count_risk_levels(&heightmap));
    println!("Day 9, part 2: {}\n", 0);
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

fn count_risk_levels(matrix: &Vec<Vec<i8>>) -> usize {
    let mut count = 0;
    let max_index_rows = matrix.len() - 1;
    let max_index_cols = matrix[0].len() - 1;

    for i in 0..=max_index_rows {
        for j in 0..=max_index_cols {
            let elem = matrix[i][j];
            
            if elem != 9 {
                count += find_elements_with_greater_adjacents(i as isize, j as isize, elem, matrix, max_index_rows as isize, max_index_cols as isize) as usize;
            }
        }
    }
    
    count
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
        //let heightmap = parse_heightmap(FILE);
        //println!("{:?}", basins(&mut heightmap));

        //assert_eq!(basins(&mut heightmap), 1134);
    }
}
