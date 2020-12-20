// Advent of Code 2020: December, 20
// Day 20: Jurassic Jigsaw

use std::collections::HashMap;

use crate::manage_input::get_tiles;

pub fn answers_day20() -> (u16, u64) {
    let filepath: &str = "inputs/day20_input.txt";
    let tiles = get_tiles(filepath);
    get_borders(&tiles);
    
    (0, 0)
}

// Dict of dicts? Enumerating each possible border.
// If flipping top, can't use unflipped bottom.
fn get_borders(tiles: &HashMap<u32, Vec<String>>) -> HashMap<u32, Vec<String>>{
    let mut borders: HashMap<u32, Vec<String>> = HashMap::new();

    for k in tiles.keys() {
        let mut elem: Vec<String> = Vec::new();
        let tile = tiles.get(k).unwrap();
        elem.push(tile.first().unwrap().to_string());   // Top.
        elem.push(tile.first().unwrap().to_string().chars().rev().collect()); // Top reverse.
        elem.push(tile.last().unwrap().to_string());    // Bottom.
        elem.push(tile.last().unwrap().to_string().chars().rev().collect());    // Bottom reverse.

        let mut  left_col: String = String::new();
        let mut  right_col: String = String::new();

        for s in tile {
            left_col.push(s.chars().nth(0).unwrap());
            right_col.push(s.chars().nth(s.len() - 1).unwrap());
        }

        elem.push(left_col.clone()); // First left column.
        elem.push(left_col.chars().rev().collect()); // First left column reverse.

        elem.push(right_col.clone()); // First right column.
        elem.push(right_col.chars().rev().collect()); // First right column reverse.

        borders.insert(*k, elem);
    }

    println!("{:?}", borders);
    borders
}

fn assemble(tiles: &HashMap<u32, Vec<String>>) {
    let square_len = f64::from(tiles.len() as i16).sqrt() as u8;
    let mut assembly: Vec<Vec<u8>> = vec![vec![0; square_len as usize]; square_len as usize];
    let mut matched: Vec<u32> = Vec::new();

    for k in tiles.keys() {
        //println!("{:?}", tiles.get(k).);
    }
}
