// Functions to parse problem inputs

use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn build_reader(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("Something went wrong opening the file");
    let reader = BufReader::new(file);

    reader
}

pub fn parse_nums(filename: &str) -> Vec<u32> {
    let mut lines = Vec::new();

    for line in build_reader(filename).lines() {
        let line = line.unwrap().parse::<u32>().expect("Not a number");
        lines.push(line);
    }

    lines
}

pub fn parse_commands(filename: &str) -> Vec<(String, u32)> {
    let mut commands = Vec::new();

    for line in build_reader(filename).lines() {
        let command = line.unwrap();
        let args: Vec<&str> = command.split_whitespace().collect();
        commands.push((args[0].to_string(), args[1].parse::<u32>().unwrap()));
    }

    commands
}

pub fn parse_diagnostic_report(filename: &str) -> Vec<Vec<char>> {
    let mut bits: Vec<Vec<char>> = Vec::new();

    // Build matrix of bits
    for line in build_reader(filename).lines() {
        bits.push(line.unwrap().chars().collect());
    }

    bits
}

pub fn transpose_matrix_chars(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let row_len = matrix[0].len();
    let mut transpose_matrix: Vec<Vec<char>> = vec![Vec::with_capacity(matrix.len()); row_len];

    for r in matrix {
        for i in 0..row_len {
            transpose_matrix[i].push(r[i]);
        }
    }

    transpose_matrix
}

pub fn parse_bingo(filename: &str) -> (Vec<u8>, Vec<Vec<Vec<u8>>>) {
    let mut reader = build_reader(filename);

    // Draw numbers
    let mut first_line = String::new();
    reader.read_line(&mut first_line).expect("Could not read line");
    let draw_numbers = first_line.trim().split(',').map(|n| n.parse::<u8>().unwrap()).collect();

    // Boards
    let mut boards: Vec<Vec<Vec<u8>>> = Vec::new();
    let mut board_lines: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines().skip(1) {
        let line = line.unwrap();

        if line.len() == 0 {
            let board_transposed = transpose_matrix_nums_u8(&board_lines);
            boards.push(board_lines);
            boards.push(board_transposed);
            board_lines = Vec::new();
        }
        else {
            let board_line: Vec<u8> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            board_lines.push(board_line);
        }
    }

    (draw_numbers, boards)
}

pub fn transpose_matrix_nums_u8(matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>>{
    let row_len = matrix[0].len();
    let mut transpose_matrix: Vec<Vec<u8>> = vec![Vec::with_capacity(matrix.len()); row_len];

    for r in matrix {
        for i in 0..row_len {
            transpose_matrix[i].push(r[i]);
        }
    }

    transpose_matrix
}

pub fn parse_vents(filename: &str) -> (Vec<(Vec<u16>, Vec<u16>)>, u16) {
    let mut vents: Vec<(Vec<u16>, Vec<u16>)> = Vec::new();
    let mut get_grid_size: Vec<u16> = Vec::new();
    
    for line in build_reader(filename).lines() {
        let l = line.unwrap();
        let mut split = l.split(" -> ");
        let left: Vec<u16> = split.next().unwrap().split(',').map(|n| n.parse::<u16>().unwrap()).collect();
        let right: Vec<u16> = split.next().unwrap().split(',').map(|n| n.parse::<u16>().unwrap()).collect();
        get_grid_size.push(left.clone().into_iter().max().unwrap());
        get_grid_size.push(right.clone().into_iter().max().unwrap());
        let extract_nums: (Vec<u16>, Vec<u16>) = (left, right);
        vents.push(extract_nums);
    }

    (vents, *get_grid_size.iter().max().unwrap() + 1)
}

pub fn parse_line_nums(filename: &str) -> [usize; 9] {
    let f = fs::read_to_string(filename).expect("Error reading file");
    let mut fishes: [usize; 9] = [0; 9];

    for timer in f.trim().split(',').map(|n| n.parse::<usize>().unwrap()) {
        fishes[timer] += 1;
    }

    fishes
}

pub fn parse_positions(filename: &str) -> Vec<u16> {
    let f = fs::read_to_string(filename).expect("Error reading file");
    let positions: Vec<u16> = f.trim().split(',').map(|n| n.parse::<u16>().unwrap()).collect();

    positions
}

pub fn parse_patterns_outputs(filename: &str) -> Vec<([String; 10], [String; 4])>{
    let mut patterns_outputs: Vec<([String; 10], [String; 4])> = Vec::new();

    for line in build_reader(filename).lines() {
        let l = line.unwrap();
        let input_sides: Vec<String> = l.split(" | ").map(|side| side.to_string()).collect();
        let patterns: [String; 10] = input_sides[0].split(" ").map(|segment| segment.to_string()).collect::<Vec<String>>().try_into().unwrap();
        let outputs: [String; 4] = input_sides[1].split(" ").map(|segment| segment.to_string()).collect::<Vec<String>>().try_into().unwrap();
        patterns_outputs.push((patterns, outputs));
    }

    patterns_outputs
}

pub fn parse_heightmap(filename: &str) -> Vec<Vec<i8>> {
    let mut height_matrix: Vec<Vec<i8>> = Vec::new();

    for line in build_reader(filename).lines() {
        let row: Vec<i8> = line.unwrap().chars().map(|n| n.to_digit(10).unwrap() as i8).collect();
        height_matrix.push(row);
    }

    height_matrix
}

pub fn parse_navigation_subsystem(filename: &str) -> Vec<String> {
    let mut navigation_subsystem: Vec<String> = Vec::new();

    for line in build_reader(filename).lines() {
        navigation_subsystem.push(line.unwrap());
    }

    navigation_subsystem
}
