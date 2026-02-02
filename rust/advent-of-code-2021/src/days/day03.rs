// Advent of Code 2021
// Day 3: Binary Diagnostic

use crate::input_handler::{parse_diagnostic_report, transpose_matrix_chars};

const FILE: &str = "inputs/day03_input.txt";

pub fn day03_answer() {
    let diagnostic_report = parse_diagnostic_report(FILE);

    let transposed_report = transpose_matrix_chars(diagnostic_report.clone());
    let power_consumption_report = read_power_consumption_report(transposed_report.clone());
    let gamma_rate = usize::from_str_radix(power_consumption_report.0.as_str(), 2).unwrap();
    let epsilon_rate = usize::from_str_radix(power_consumption_report.1.as_str(), 2).unwrap();

    println!("Day 3, part 1: {}", gamma_rate * epsilon_rate);

    let read_life_support_rating_report = read_life_support_rating_report(diagnostic_report);
    let oxigen_generator_rating = usize::from_str_radix(read_life_support_rating_report.0.as_str(), 2).unwrap();
    let co2_scrubber_rating = usize::from_str_radix(read_life_support_rating_report.1.as_str(), 2).unwrap();
    
    println!("Day 3, part 2: {:?}\n", oxigen_generator_rating * co2_scrubber_rating);
}

fn read_power_consumption_report(transposed_report: Vec<Vec<char>>) -> (String, String) {
    let mut gamma: Vec<char> = Vec::new();
    let mut epsilon: Vec<char> = Vec::new();

    for r in transposed_report {
        let count_ones = r.iter().filter(|&b| *b == '1').count();
        let count_zeros = r.len() - count_ones;

        if count_ones > count_zeros {
            gamma.push('1');
            epsilon.push('0');
        }
        else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

   (gamma.into_iter().collect(),  epsilon.into_iter().collect())
}

fn read_life_support_rating_report(diagnostic_report: Vec<Vec<char>>) -> (String, String) {
    let mut oxigen_generator_rating = diagnostic_report.clone();
    let mut co2_scrubber_rating = diagnostic_report;

    let mut next_bit_index = 0;

    while next_bit_index < oxigen_generator_rating[0].len() {
        let len_oxigen_generator_rating = oxigen_generator_rating.len();
        let count_ones = transpose_matrix_chars(oxigen_generator_rating.clone())[next_bit_index].iter().filter(|&b| *b == '1').count();
        let count_zeros = len_oxigen_generator_rating - count_ones;
        let mut bit_criteria = '1';
        let last = oxigen_generator_rating[len_oxigen_generator_rating - 1].clone();

        if count_ones < count_zeros {
            bit_criteria = '0';
        }

        oxigen_generator_rating.retain(|bits| bits[next_bit_index] == bit_criteria);
        
        if oxigen_generator_rating.len() == 0 {
            oxigen_generator_rating.push(last);
            break;
        }

        next_bit_index += 1;
    }

    next_bit_index = 0;

    while next_bit_index < co2_scrubber_rating[0].len() {
        let len_co2_scrubber_rating = co2_scrubber_rating.len();
        let count_ones = transpose_matrix_chars(co2_scrubber_rating.clone())[next_bit_index].iter().filter(|&b| *b == '1').count();
        let count_zeros = len_co2_scrubber_rating - count_ones;
        let mut bit_criteria = '1';
        let last = co2_scrubber_rating[len_co2_scrubber_rating - 1].clone();

        if count_ones >= count_zeros {
            bit_criteria = '0';
        }

        co2_scrubber_rating.retain(|bits| bits[next_bit_index] == bit_criteria);
        
        if co2_scrubber_rating.len() == 0 {
            co2_scrubber_rating.push(last);
            break;
        }

        next_bit_index += 1;
    }
    
    (oxigen_generator_rating.into_iter().nth(0).unwrap().into_iter().collect(), co2_scrubber_rating.into_iter().nth(0).unwrap().into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day03_input_test.txt";

    #[test]
    fn day03_part1_test() {
        let diagnostic_report = parse_diagnostic_report(FILE);
        let power_consumption_report = read_power_consumption_report(transpose_matrix_chars(diagnostic_report));
        let gamma_rate = usize::from_str_radix(power_consumption_report.0.as_str(), 2).unwrap();
        let epsilon_rate = usize::from_str_radix(power_consumption_report.1.as_str(), 2).unwrap();

        assert_eq!(gamma_rate * epsilon_rate, 198);
    }

    #[test]
    fn day03_part2_test() {
        let diagnostic_report = parse_diagnostic_report(FILE);
        let read_life_support_rating_report = read_life_support_rating_report(diagnostic_report);
        let oxigen_generator_rating = usize::from_str_radix(read_life_support_rating_report.0.as_str(), 2).unwrap();
        let co2_scrubber_rating = usize::from_str_radix(read_life_support_rating_report.1.as_str(), 2).unwrap();

        assert_eq!(oxigen_generator_rating * co2_scrubber_rating, 230);
    }
}
