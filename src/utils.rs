use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_numbers(lines: io::Lines<io::BufReader<File>>) -> Vec<i32> {
    // Consumes the iterator, returns an (Optional) String
    let mut numbers = Vec::new();
    for line in lines {
        if let Ok(ip) = line {
            let i = ip.parse::<i32>().unwrap();
            numbers.push(i);
        }
    }

    numbers
}

pub fn read_strings(lines: io::Lines<io::BufReader<File>>) -> Vec<String> {
    // Consumes the iterator, returns an (Optional) String
    let mut strings = Vec::new();
    for line in lines {
        if let Ok(ip) = line {
            strings.push(ip);
        }
    }

    strings
}

// Based on https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
