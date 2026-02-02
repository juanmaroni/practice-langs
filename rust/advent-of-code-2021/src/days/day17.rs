// Advent of Code 2021
// Day 17: Trick Shot

use crate::input_handler::parse_target_area;

const FILE: &str = "inputs/day17_input.txt";

pub fn day17_answer() {
    let (target_area_x, target_area_y) = parse_target_area(FILE);
    let answers = find_velocities(target_area_x, target_area_y);
    
    println!("Day 17, part 1: {}", answers.0);

    println!("Day 17, part 2: {}\n", answers.1);
}

fn find_velocities(target_area_x: (i32, i32), target_area_y: (i32, i32)) -> (i32, i32) {
    let mut max_y = 0;
    let mut within_count = 0;

    for x in -500..500 {
        for y in -500..500 {
            let mut curr_pos = (0, 0);
            let mut curr_vel = (x, y);

            let mut y = 0;

            for _ in 0..1000 {
                curr_pos.0 += curr_vel.0;
                curr_pos.1 += curr_vel.1;

                if curr_pos.1 > y {
                    y = curr_pos.1;
                }

                if curr_vel.0 < 0 {
                    curr_vel.0 += 1;
                }
                else if curr_vel.0 > 0 {
                    curr_vel.0 -= 1;
                }

                curr_vel.1 -= 1;

                if target_area_x.0 <= curr_pos.0 && curr_pos.0 <= target_area_x.1 && target_area_y.0 <= curr_pos.1 && curr_pos.1 <= target_area_y.1 {
                    within_count += 1;

                    if y > max_y {
                        max_y = y;
                    }
    
                    break;
                }
            }
        }
    }

    (max_y, within_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day17_input_test.txt";

    #[test]
    fn day17_part1_test() {
        let (target_area_x, target_area_y) = parse_target_area(FILE);
        
        assert_eq!(find_velocities(target_area_x, target_area_y).0, 45);
    }

    #[test]
    fn day17_part2_test() {
        let (target_area_x, target_area_y) = parse_target_area(FILE);
        
        assert_eq!(find_velocities(target_area_x, target_area_y).1, 112);
    }
}
