// Advent of Code 2020: December, 8
// Day 8: Handheld Halting

use crate::manage_input::parse_instructions;

pub fn answers_day8() -> (isize, isize) {
    let filepath = "inputs/day08_input.txt";
    let instructions = parse_instructions(filepath);

    (execute_instructions(&instructions), fix_loop(&instructions))
}

fn execute_instructions(instructions: &Vec<(String, i16)>) -> isize {
    let mut executed: Vec<usize> = Vec::new();
    let mut acc: isize = 0;
    let mut i: usize = 0;

    while i <= instructions.len() && !executed.contains(&i) {
        executed.push(i);

        if instructions[i].0 == "acc" {
            acc += instructions[i].1 as isize;
            i += 1;
        }
        else if instructions[i].0 == "jmp" {
            i = jmp(i, instructions[i].1);
        }
        else {
            i += 1;
        }
    }
    
    acc
}

fn fix_loop(instructions: &Vec<(String, i16)>) -> isize {
    let instructions_len = instructions.len();

    for j in 0..instructions_len {
        let mut executed: Vec<usize> = Vec::new();
        let mut acc: isize = 0;
        let mut i: usize = 0;
        
        while i < instructions.len() && !executed.contains(&i) {
            executed.push(i);

            if j == i {
                // If the instruction is "acc", it won't change the result, skip.
                if instructions[i].0 == "acc" {
                    break;
                }
                else if instructions[i].0 == "nop" {
                    i = jmp(i, instructions[i].1);
                }
                else {
                    i += 1;
                }
            }
            else {
                if instructions[i].0 == "acc" {
                    acc += instructions[i].1 as isize;
                    i += 1;
                }
                else if instructions[i].0 == "jmp" {
                    i = jmp(i, instructions[i].1);
                }
                else {
                    i += 1;
                }
            }
        }

        if i == instructions_len {
            return acc;
        }
    }
    
    0   // Impossible to fix the loop.
}

fn jmp(mut i: usize, instruction: i16) -> usize {
    if instruction < 0 {
        i = (i as isize + instruction as isize) as usize;
    }
    else {
        i += instruction as usize;
    }

    i
}