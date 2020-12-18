// Advent of Code 2020: December, 18
// Day 18: Operation Order

use crate::manage_input::get_expressions;

// First time using a PEG.

pub fn answers_day18() -> (u64, u64) {
    let filepath: &str = "inputs/day18_input.txt";
    let expressions = get_expressions(filepath);
    
    calculate_expression(&expressions)
}

fn calculate_expression(expr: &Vec<String>) -> (u64, u64) {
    (expr.iter().map(|c| operation_parser::straight(c).unwrap()).sum::<u64>(),
    expr.iter().map(|c| operation_parser::sum_precedence(c).unwrap()).sum::<u64>())
}

peg::parser! {
    grammar operation_parser() for str {
        pub rule straight() -> u64= precedence! {
            x:(@) _ "+" _ y:@ { x + y }
            x:(@) _ "*" _ y:@ { x * y }
            --
            "(" _ e:straight() _ ")" { e }
            n:number() { n }
        }

        pub rule sum_precedence() -> u64= precedence! {
            x:(@) _ "*" _ y:@ { x * y }
            --
            x:(@) _ "+" _ y:@ { x + y }
            --
            "(" _ e:sum_precedence() _ ")" { e }
            n:number() { n }
        }

        rule number() -> u64 = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule _() = " "?
    }
}
