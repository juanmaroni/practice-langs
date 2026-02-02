// Advent of Code 2021
// Day 10: Syntax Scoring

use crate::input_handler::parse_navigation_subsystem;

const FILE: &str = "inputs/day10_input.txt";

pub fn day10_answer() {
    let mut navigation_subsystem = parse_navigation_subsystem(FILE);

    println!("Day 10, part 1: {}", score_syntax_errors(&mut navigation_subsystem));
    println!("Day 10, part 2: {}\n", score_syntax_completions(&mut navigation_subsystem));
}

fn find_corrupts(line: &String) -> Vec<char> {
    let mut stack: Vec<char> = Vec::new();
    let mut corrupts: Vec<char> = Vec::new();
    let opening_chars: [char; 4] = ['(', '{', '[', '<'];

    for c in line.chars() {
        if opening_chars.contains(&c) {
            stack.push(c);
        }
        else {
            let last = stack.pop().unwrap();

            if last == '(' {
                if c != ')' {
                    corrupts.push(c);
                }
            }
            else if last == '{' {
                if c != '}' {
                    corrupts.push(c);
                }
            }
            else if last == '[' {
                if c != ']' {
                    corrupts.push(c);
                }
            }
            else if last == '<' {
                if c != '>' {
                    corrupts.push(c);
                }
            }
        }
    }

    corrupts
}

fn match_points_invalid(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score_syntax_errors(nav_subsystem: &mut Vec<String>) -> usize {
    let mut score = 0;

    nav_subsystem.retain(|line| {
        let errors = find_corrupts(line);
        if errors.len() == 0 {
            return true;
        }
        else {
            score += errors.iter().map(|c| match_points_invalid(*c)).sum::<usize>();
            return false;
        }
    });
    
    score
}

fn find_completions(line: &String) -> Vec<char> {
    let mut stack: Vec<char> = Vec::new();
    let opening_chars: [char; 4] = ['(', '{', '[', '<'];

    for c in line.chars() {
        if opening_chars.contains(&c) {
            stack.push(c);
        }
        else {
            let last = stack.last().unwrap();

            if *last == '(' && c == ')' {
                stack.pop();
            }
            else if *last == '{' && c == '}' {
                stack.pop();
            }
            else if *last == '[' && c == ']' {
                stack.pop();
            }
            else if *last == '<' && c == '>' {
                stack.pop();
            }
        }
    }

    stack.into_iter().rev().collect::<Vec<char>>()
}

fn match_points_completion(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn score_syntax_completions(nav_subsystem: &mut Vec<String>) -> usize {
    let mut scores: Vec<usize> = Vec::new();

    if nav_subsystem.len() == 0 {
        return 0;
    }

    for line in nav_subsystem {
        let mut score = 0;
        let completions = find_completions(line);

        for c in completions {
            score = score * 5 + match_points_completion(c);
        }
        
        scores.push(score);
    }

    scores.sort();
    let len_scores = scores.len();

    if len_scores == 0 {
        return 0;
    }

    scores[len_scores / 2]
}


#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day10_input_test.txt";

    #[test]
    fn day10_part1_test() {
        let mut navigation_subsystem = parse_navigation_subsystem(FILE);

        assert_eq!(score_syntax_errors(&mut navigation_subsystem), 26397);
    }

    #[test]
    fn day10_part2_test() {
        let mut navigation_subsystem = parse_navigation_subsystem(FILE);
        score_syntax_errors(&mut navigation_subsystem);

        assert_eq!(score_syntax_completions(&mut vec![]), 0);
        assert_eq!(score_syntax_completions(&mut navigation_subsystem), 288957);
    }
}
