// Advent of Code 2021
// Day 20: Trench Map

use crate::input_handler::parse_algorithm_and_image;

const FILE: &str = "inputs/day20_input.txt";

pub fn day20_answer() {
    let (algorithm, image) = parse_algorithm_and_image(FILE);
    
    println!("Day 20, part 1: {}", iterate_twice(&algorithm, &image));

    println!("Day 20, part 2: {}\n", 0);
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

fn enhance(matrix: &Vec<char>, algorithm: &Vec<char>) -> char {
    let bits = matrix.iter().map(|c| if c == &'.' {'0'} else {'1'}).collect::<String>();

    algorithm[usize::from_str_radix(&bits, 2).unwrap()]
}

fn iterate_twice(algorithm: &Vec<char>, image: &Vec<Vec<char>>) -> u16 {
    let mut image = image.clone();
    
    for _ in 0..2 {
        add_padding(&mut image, 2, '.');
        
        let mut new_image = image.clone();

        for i in 1..image.len() -1 {
            for j in 1..image[0].len() - 1 {
                let matrix = vec!(image[i-1][j-1], image[i-1][j], image[i-1][j+1], image[i][j-1], image[i][j], image[i][j+1], image[i+1][j-1], image[i+1][j], image[i+1][j+1]);
                new_image[i][j] = enhance(&matrix, algorithm);
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
        
        assert_eq!(iterate_twice(&algorithm, &image), 35);
    }

    #[test]
    fn day20_part2_test() {
        
        //assert_eq!(, 112);
    }
}
