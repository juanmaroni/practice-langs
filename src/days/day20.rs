// Advent of Code 2021
// Day 20: Trench Map

use crate::input_handler::parse_algorithm_and_image;

const FILE: &str = "inputs/day20_input.txt";

pub fn day20_answer() {
    let (algorithm, image) = parse_algorithm_and_image(FILE);
    
    println!("Day 20, part 1: {}", iterate_twice(&algorithm, &image, 2));

    println!("Day 20, part 2: {}\n", iterate_twice(&algorithm, &image, 50));
}

fn add_padding(image: &mut Vec<Vec<char>>, size: usize, c: char) {
    for r in image.iter_mut() {
        for _ in 0..size {
            r.insert(0, c);
            r.push(c);
        }
    }

    let size_line = image.len() + size * 2;

    for _ in 0..size {
        image.insert(0, (0..size_line).map(|_| c).collect::<String>().chars().collect::<Vec<_>>());
        image.push((0..size_line).map(|_| c).collect::<String>().chars().collect::<Vec<_>>());
    }
}

fn enhance(image: &Vec<Vec<char>>, r: usize, c: usize, algorithm: &Vec<char>, it: usize) -> char {
    let matrix = vec!(image[r-1][c-1], image[r-1][c], image[r-1][c+1], image[r][c-1], image[r][c], image[r][c+1], image[r+1][c-1], image[r+1][c], image[r+1][c+1]);
    let mut bits = String::new();

    for e in matrix {
        if e == '#' {
            bits.push('1');
        }
        else if e  == '.' {
                bits.push('0');
        }
        else {
            if it % 2 == 0 {
                bits.push('0');
            }
            else {
                bits.push('1');
            }
        }
    }

    algorithm[usize::from_str_radix(&bits, 2).unwrap()]
}

fn iterate_twice(algorithm: &Vec<char>, image: &Vec<Vec<char>>, iterations: usize) -> u16 {
    let mut image = image.clone();
    
    for it in 0..iterations {
        add_padding(&mut image, 2, '-');
        
        let mut new_image = image.clone();

        for i in 1..image.len() -1 {
            for j in 1..image[0].len() - 1 {
                new_image[i][j] = enhance(&image, i, j, algorithm, it);
            }
        }

        image = new_image;
    }

    image.iter().flat_map(|v| v.iter().filter(|c| c == &&'#')).count() as u16  
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day20_input_test.txt";

    #[test]
    fn day20_part1_test() {
        let (algorithm, image) = parse_algorithm_and_image(FILE);

        assert_eq!(iterate_twice(&algorithm, &image, 2), 5326);
    }

    #[test]
    fn day20_part2_test() {
        let (algorithm, image) = parse_algorithm_and_image(FILE);

        assert_eq!(iterate_twice(&algorithm, &image, 50), 17096);
    }
}
