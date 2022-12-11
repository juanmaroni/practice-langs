// Advent of Code 2022
// Day 11: Monkey in the Middle

use aoc2022::{Day, Part, Answer};
use itertools::Itertools;

const FILE: &str = "inputs/real/day11_input.txt";

#[derive(Debug, Clone)]
struct Monkey {
    number: u8, // Monkey number
    items: Vec<u32>, // Starting items
    operation: (char, String), // Operation: new
    div_by: u16, // Test: divisible by
    test_true: usize, // If true: throw to monkey X
    test_false: usize, // If false: throw to monkey Y
    inspections: u64, // Number of inspections
}

pub fn print_answers() {
    let mut day = Day::new(11, FILE.to_string());

    let monkeys = get_info_monkeys();

    day.first_answer = Some(Answer::Num(calc_monkey_business(monkeys.clone(), 20)));
    day.second_answer = Some(Answer::Num(calc_monkey_business_relief(monkeys.clone(), 10000)));

    day.print_answer(day.day_number, Part::One, &day.first_answer);
    day.print_answer(day.day_number, Part::Two, &day.second_answer);
}

fn get_info_monkeys() -> Vec<Monkey> {
    let m0 = Monkey { number: 0, items: vec![83, 62, 93], operation: ('*', "17".to_string()), div_by: 2, test_true: 1, test_false: 6, inspections: 0 };
    let m1 = Monkey { number: 1, items: vec![90, 55], operation: ('+', "1".to_string()), div_by: 17, test_true: 6, test_false: 3, inspections: 0 };
    let m2 = Monkey { number: 2, items: vec![91, 78, 80, 97, 79, 88], operation: ('+', "3".to_string()), div_by: 19, test_true: 7, test_false: 5, inspections: 0 };
    let m3 = Monkey { number: 3, items: vec![64, 80, 83, 89, 59], operation: ('+', "5".to_string()), div_by: 3, test_true: 7, test_false: 2, inspections: 0 };
    let m4 = Monkey { number: 4, items: vec![98, 92, 99, 51], operation: ('*', "old".to_string()), div_by: 5, test_true: 0, test_false: 1, inspections: 0 };
    let m5 = Monkey { number: 5, items: vec![68, 57, 95, 85, 98, 75, 98, 75], operation: ('+', "2".to_string()), div_by: 13, test_true: 4, test_false: 0, inspections: 0 };
    let m6 = Monkey { number: 6, items: vec![74], operation: ('+', "4".to_string()), div_by: 7, test_true: 3, test_false: 2, inspections: 0 };
    let m7 = Monkey { number: 7, items: vec![68, 64, 60, 68, 87, 80, 82], operation: ('*', "19".to_string()), div_by: 11, test_true: 4, test_false: 5, inspections: 0 };

    vec![m0, m1, m2, m3, m4, m5, m6, m7]
}

// Part 1
fn calc_monkey_business(monkeys: Vec<Monkey>, rounds: usize) -> u64 {
    let mut monkeys = monkeys;

    for _ in 0..rounds {
        monkeys = inspect_and_throw_round(monkeys);
    }

    monkeys.iter()
        .map(|m| m.inspections as u64)
        .sorted()
        .rev()
        .take(2)
        .product::<u64>()
}

// Part 2
fn calc_monkey_business_relief(monkeys: Vec<Monkey>, rounds: usize) -> u64 {
    let mut monkeys = monkeys;
    let relief = monkeys.clone().iter().map(|m| m.div_by as u64).product::<u64>();

    for _ in 0..rounds {
        monkeys = inspect_and_throw_round_relief(monkeys, relief);
    }

    monkeys.iter()
        .map(|m| m.inspections as u64)
        .sorted()
        .rev()
        .take(2)
        .product::<u64>()
}

// Helping function for Part 1
fn inspect_and_throw_round(mut monkeys: Vec<Monkey>) -> Vec<Monkey> {
    let monkeys_len = monkeys.len();

    for n in 0..monkeys_len {
        let mut new_monkeys = monkeys.clone();
        let mut m = monkeys[n].clone();

        for i in &mut m.items {
            let mut item = *i;

            if m.operation.1 == "old" {
                if m.operation.0 == '+' {
                    item += item;
                } else if m.operation.0 == '*' {
                    item *= item;
                }
            } else {
                if m.operation.0 == '+' {
                    item += m.operation.1.parse::<u32>().unwrap();
                } else if m.operation.0 == '*' {
                    item *= m.operation.1.parse::<u32>().unwrap();
                }
            }

            // Worry level / 3, rounded down to nearest integer
            item = (item as f32 / 3.0).floor() as u32;

            if item % m.div_by as u32 == 0 {
                new_monkeys[m.test_true].items.push(item);
            } else {
                new_monkeys[m.test_false].items.push(item);
            }

            new_monkeys[m.number as usize].items.remove(0);
            new_monkeys[m.number as usize].inspections += 1;
        }

        monkeys = new_monkeys;
    }

    monkeys
}

// Helping function for Part 2
fn inspect_and_throw_round_relief(mut monkeys: Vec<Monkey>, relief: u64) -> Vec<Monkey> {
    let monkeys_len = monkeys.len();

    for n in 0..monkeys_len {
        let mut new_monkeys = monkeys.clone();
        let mut m = monkeys[n].clone();

        for i in &mut m.items {
            let mut item = *i as u64;

            if m.operation.1 == "old" {
                if m.operation.0 == '+' {
                    item += item;
                } else if m.operation.0 == '*' {
                    item *= item;
                }
            } else {
                if m.operation.0 == '+' {
                    item += m.operation.1.parse::<u64>().unwrap();
                } else if m.operation.0 == '*' {
                    item *= m.operation.1.parse::<u64>().unwrap();
                }
            }

            // Worry level
            item = item % relief as u64;

            if item % m.div_by as u64 == 0 {
                new_monkeys[m.test_true].items.push(item as u32);
            } else {
                new_monkeys[m.test_false].items.push(item as u32);
            }

            new_monkeys[m.number as usize].items.remove(0);
            new_monkeys[m.number as usize].inspections += 1;
        }
        
        monkeys = new_monkeys;
    }

    monkeys
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/tests/day11_input_test.txt";

    fn get_info_monkeys_test() -> Vec<Monkey> {
        let m0 = Monkey { number: 0, items: vec![79, 98], operation: ('*', "19".to_string()), div_by: 23, test_true: 2, test_false: 3, inspections: 0 };
        let m1 = Monkey { number: 1, items: vec![54, 65, 75, 74], operation: ('+', "6".to_string()), div_by: 19, test_true: 2, test_false: 0, inspections: 0 };
        let m2 = Monkey { number: 2, items: vec![79, 60, 97], operation: ('*', "old".to_string()), div_by: 13, test_true: 1, test_false: 3, inspections: 0 };
        let m3 = Monkey { number: 3, items: vec![74], operation: ('+', "3".to_string()), div_by: 17, test_true: 0, test_false: 1, inspections: 0 };
    
        vec![m0, m1, m2, m3]
    }

    #[test]
    fn day11_part1_test() {
        let mut day = Day::new(11, FILE.to_string());
        let monkeys = get_info_monkeys_test();
        let ans = calc_monkey_business(monkeys, 20);

        assert_eq!(ans, 10605);

        day.first_answer = Some(Answer::Num(ans));
    }

    #[test]
    fn day11_part2_test() {
        let mut day = Day::new(11, FILE.to_string());
        let monkeys = get_info_monkeys_test();
        let ans = calc_monkey_business_relief(monkeys, 10000);

        assert_eq!(ans, 2713310158);

        day.second_answer = Some(Answer::Num(ans));
    }
}
