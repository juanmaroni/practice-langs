// Advent of Code 2020: December, 23
// Day 23: Crab Cups

use crate::manage_input::get_cups;

pub fn answers_day23() -> (String, u64) {
    let filepath: &str = "inputs/day23_input.txt";
    let cups = get_cups(filepath);

    (str_answer1(&play_game(&cups, 100)), 0)
}

fn play_game(cups: &Vec<u8>, rounds: usize) -> Vec<u8> {
    let mut play_cups = cups.clone();

    // I wanted to play around the rotation method.
    // Every round, I move the current element to the last position.
    for _ in 0..rounds {
        //println!("move: {:?}", i + 1);
        //println!("cups: {:?}", play_cups);
        let current = play_cups[0];
        
        let removed: [u8; 3] = [play_cups[1], play_cups[2], play_cups[3]];
        play_cups.drain(1..4);
        //println!("removed cups: {:?}", &play_cups);
        //println!("pikcup: {:?}", removed);

        let destination = find_next(&play_cups, removed, current);
        //println!("destination: {:?}", destination);

        let destination_index = (play_cups.iter().position(|&c| c == destination).unwrap()) as usize;
        //println!("dest index {:?}\n", destination_index);
        
        play_cups.rotate_left(1);
        play_cups.splice(destination_index..destination_index, removed.iter().cloned());
    }

    play_cups
}

fn find_next(cups: &Vec<u8>, removed: [u8; 3], current: u8) -> u8 {
    let mut next = current - 1;

    if next == 0 {
        next = *(cups.iter().max().unwrap());
    }

    while removed.contains(&next) {
        next -= 1;

        if next == 0 {
            next = *(cups.iter().max().unwrap());
        }
    }

    next
}

fn str_answer1(cups: &Vec<u8>) -> String {
    let mut play_cups = cups.clone();
    let index_1 = play_cups.iter().position(|&c| c == 1).unwrap();
    play_cups.rotate_left(index_1);

    play_cups.iter().skip(1).map(ToString::to_string).collect()
}
