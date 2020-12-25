// Advent of Code 2020: December, 1
// Day 1: Report Repair

use itertools::Itertools;

use crate::manage_input::parse_expense_report;

const YEAR: u16 = 2020;

pub fn answers_day1() -> (u32, u32) {
    let expense_report = parse_expense_report("inputs/day01_input.txt", YEAR);

    (find_sum_2_numbers(expense_report.clone()), find_sum_3_numbers(expense_report))
}

fn find_sum_2_numbers(expense_report: Vec<u16>) -> u32 {
    let (n, m) = expense_report.iter()
        .cartesian_product(&expense_report)
        .find(|(n, m)| *n + *m == YEAR)
        .expect("Not found");

    u32::from(*n) * u32::from(*m)
}

fn find_sum_3_numbers(expense_report: Vec<u16>) -> u32 {
    let ((n, m), k) = expense_report.iter()
        .cartesian_product(&expense_report)
        .cartesian_product(&expense_report)
        .find(|((n, m), k)| *n + *m + *k == YEAR)
        .expect("Not found");

    u32::from(*n) * u32::from(*m) * u32::from(*k)
}
