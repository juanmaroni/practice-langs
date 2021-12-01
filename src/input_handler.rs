use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file_lines_as_nums(filename: &str) -> Vec<u32> {
    let file = File::open(filename).expect("Something went wrong opening the file");
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap().parse::<u32>().expect("Not a number");
        //println!("{}", line);
        lines.push(line);
    }

    lines
}
