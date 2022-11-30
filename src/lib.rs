// Common code for every day
use std::fmt;
use std::fs::File;
use std::io::BufReader;

// Answers can be of two types: unsigned integers or strings
#[derive(Debug)]
pub enum Answer {
    Num(u64),
    Str(String),
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match *self {
            Answer::Num(ref inner) => ::std::fmt::Display::fmt(inner, f),
            Answer::Str(ref inner) => ::std::fmt::Display::fmt(inner, f),
        }
    }
}

// Part of the Day (1 or 2)
pub enum Part {
    One,
    Two,
}

impl fmt::Display for Part {
    // Print Part as a number
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
        match *self {
            Part::One => f.write_str("1"),
            Part::Two => f.write_str("2"),
        }
    }
}

// An AoC day is compossed of:
// - Day number.
// - A file (passing the path).
// - Two answers (integers or strings).
#[derive(Debug)]
pub struct Day {
    pub day_number: u8,
    pub file: String,
    pub first_answer: Option<Answer>,
    pub second_answer: Option<Answer>,
}

impl Day {
    // Create a new Day with the filepath
    pub fn new(day_number: u8, filepath: String) -> Day {
        Day {
            day_number: day_number,
            file: filepath,
            first_answer: None,
            second_answer: None,
        }
    }

    // Create a reader for the file
    pub fn read_file(&self)  -> BufReader<File> {
        let file = File::open(&self.file)
                            .expect("Something went wrong opening the file");
        
        BufReader::new(file)
    }

    // Print answer, including a print text like "Day 1, part 1:"
    pub fn print_answer(&self, day_number: u8, part: Part, ans: &Option<Answer>) {
        println!("Day {}, part {}: {}", day_number, part, ans.as_ref().unwrap());
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day_attributes_test() {
        let mut test_day = Day::new(1, String::from("inputs/tests/day01_input_test.txt"));
        test_day.first_answer = Some(Answer::Num(u64::MAX));

        // The Day has a filepath set
        assert!(!test_day.file.len() > 0);

        // The Day has the first answer set
        assert!(!test_day.first_answer.is_none());

        // The Day does not have the second answer set
        assert!(test_day.second_answer.is_none());

        // Try to read the file (it can panic with a bad path)
        test_day.read_file();
    }

    #[test]
    fn print_test() {
        let mut test_day = Day::new(1, String::from("filepath"));
        test_day.first_answer = Some(Answer::Num(u64::MAX));
        test_day.second_answer = Some(Answer::Str(String::from("Answer")));

        // Print first answer for this Day
        test_day.print_answer(test_day.day_number, Part::One, &test_day.first_answer);

        // Print second answer for this Day
        test_day.print_answer(test_day.day_number, Part::Two, &test_day.first_answer);
    }
}
