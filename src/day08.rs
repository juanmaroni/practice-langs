// Advent of Code 2020: December, 8
// Day 8: Handheld Halting

use crate::manage_input;

pub fn answers_day8() -> (u16, u16) {
    let filepath: &str = "inputs/day08_input.txt";
    execute_instructions(filepath);

    (0, 0)
}

fn execute_instructions(filepath: &str) -> isize {
    let instructions = manage_input::read_instructions(filepath);
    let mut executed: Vec<usize> = Vec::new();
    let mut acc: isize = 0;
    let mut i: usize = 0;

    loop {
        if i > instructions.len() || executed.contains(&i) {
            break;
        }

        executed.push(i);
        //println!("{:?}", instructions[i]);

        if instructions[i].0 == "acc" {
            acc += instructions[i].1 as isize;
            i += 1;
        }
        else if instructions[i].0 == "jmp" {
            if instructions[i].1 < 0 {
                // This was very tricky, because it went negative and caused overflow.
                // I had to multicast to operate with negatives.
                i = (i as isize + instructions[i].1 as isize) as usize;
            }
            else {
                i += instructions[i].1 as usize;
            }
        }
        else {
            i += 1;
        }

        //println!(" I :{:?}", i);
    }
    
    println!("{:?}", acc);
    acc
}
