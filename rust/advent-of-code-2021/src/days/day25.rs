// Advent of Code 2021
// Day 25: Sea Cucumber

use crate::input_handler::parse_sea_cucumbers;

const FILE: &str = "inputs/day25_input.txt";

pub fn day25_answer() {
    let sea_cucumbers = parse_sea_cucumbers(FILE);
    
    println!("Day 25, part 1: {}", move_sea_cucumbers(&sea_cucumbers));

    println!("Day 25, part 2: MERRY CHRISTMAS!! 🎅\n",);
}

fn move_sea_cucumbers(sea_cucumbers: &Vec<Vec<char>>) -> u16 {
    let len_row = sea_cucumbers.len();
    let len_col = sea_cucumbers[0].len();
    let mut sea_cucumbers = sea_cucumbers.clone();
    let mut steps = 0;

    loop {
        let mut next_sea_cucumbers_left = sea_cucumbers.clone();

        for r in 0..len_row {
            for c in 0..len_col {
                let next_col = if c == len_col - 1 { 0 } else { c + 1 };

                if sea_cucumbers[r][c] == '>' {
                    if sea_cucumbers[r][next_col] == '.' {
                        next_sea_cucumbers_left[r][c] = '.';
                        next_sea_cucumbers_left[r][next_col] = '>';
                    }
                }
            }
        }

        let mut next_sea_cucumbers_down = next_sea_cucumbers_left.clone();

        for r in 0..len_row {
            let next_row = if r == len_row - 1 { 0 } else { r + 1 };

            for c in 0..len_col {
                if next_sea_cucumbers_left[r][c] == 'v' {
                    if next_sea_cucumbers_left[next_row][c] == '.' {
                        next_sea_cucumbers_down[r][c] = '.';
                        next_sea_cucumbers_down[next_row][c] = 'v';
                    }
                }
            }
        }

        steps += 1;

        if sea_cucumbers == next_sea_cucumbers_down {
            break;
        }

        sea_cucumbers = next_sea_cucumbers_down;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day25_input_test.txt";

    #[test]
    fn day25_part1_test() {
        let sea_cucumbers = parse_sea_cucumbers(FILE);
        
        assert_eq!(move_sea_cucumbers(&sea_cucumbers), 58);
    }

    #[test]
    fn day25_part2_test() {
        
    }
}
