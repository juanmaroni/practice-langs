// Advent of Code 2022
// Day 11: Monkey in the Middle

use itertools::Itertools;
use aoc2022::{Day, Part, Answer};

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

impl Monkey {
    fn new(number: u8, items: Vec<u32>, operation: (char, &str), div_by: u16, test_true: usize, test_false: usize) -> Monkey {
        Monkey {
            number: number,
            items: items,
            operation: (operation.0, String::from(operation.1)),
            div_by: div_by,
            test_true: test_true,
            test_false: test_false,
            inspections: 0,
        }
    }
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
    let m0 = Monkey::new(0, vec![83, 62, 93], ('*', "17"), 2, 1, 6);
    let m1 = Monkey::new(1, vec![90, 55], ('+', "1"), 17, 6, 3);
    let m2 = Monkey::new(2, vec![91, 78, 80, 97, 79, 88], ('+', "3"), 19, 7, 5);
    let m3 = Monkey::new(3, vec![64, 80, 83, 89, 59], ('+', "5"), 3, 7, 2);
    let m4 = Monkey::new(4, vec![98, 92, 99, 51], ('*', "old"), 5, 0, 1);
    let m5 = Monkey::new(5, vec![68, 57, 95, 85, 98, 75, 98, 75], ('+', "2"), 13, 4, 0);
    let m6 = Monkey::new(6, vec![74], ('+', "4"), 7, 3, 2);
    let m7 = Monkey::new(7, vec![68, 64, 60, 68, 87, 80, 82], ('*', "19"), 11, 4, 5);

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
    let relief = monkeys.clone()
        .iter()
        .map(|m| m.div_by as u64)
        .product::<u64>();

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
        let m0 = Monkey::new(0, vec![79, 98], ('*', "19"), 23, 2, 3);
        let m1 = Monkey::new(1, vec![54, 65, 75, 74], ('+', "6"), 19, 2,  0);
        let m2 = Monkey::new(2, vec![79, 60, 97], ('*', "old"), 13, 1, 3);
        let m3 = Monkey::new(3, vec![74], ('+', "3"), 17, 0, 1);
    
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
