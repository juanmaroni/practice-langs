// Functions to parse problem inputs

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

pub fn parse_bingo(filename: &str) -> (Vec<u8>, Vec<Vec<u8>>) { //
    let mut draw_numbers: Vec<u8> = Vec::new();
    let mut all_possible_lines: Vec<Vec<u8>> = Vec::new();

    // First line: draw numbers
    for line in build_reader(filename).lines() {
        draw_numbers = line.unwrap().split(",").map(|s| s.parse().unwrap()).collect();
        break;
    }

    // All other lines
    let mut board_table: Vec<Vec<u8>> = Vec::new();

    for line in build_reader(filename).lines().skip(2) {
        let line = line.unwrap();
        
        if line.len() == 0 {
            let transposed_table: Vec<Vec<u8>> = transpose_matrix_nums_u8(board_table.clone());

            for b_line in board_table {
                all_possible_lines.push(b_line);
            }

            for b_line in transposed_table {
                all_possible_lines.push(b_line);
            }

            board_table = Vec::new();
        }
        else {
            let board_line: Vec<u8> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            board_table.push(board_line);
        }
    }

    (draw_numbers, all_possible_lines)
}

pub fn transpose_matrix_nums_u8(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let row_len = matrix[0].len();
    let mut transpose_matrix: Vec<Vec<u8>> = vec![Vec::with_capacity(matrix.len()); row_len];

    for r in matrix {
        for i in 0..row_len {
            transpose_matrix[i].push(r[i]);
        }
    }

    transpose_matrix
}
