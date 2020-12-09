use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn get_strings(filename: &str) -> Vec<String> {
    if let Ok(lines) = read_lines(filename) {
        read_strings(lines)
    } else {
        panic!("Could not open file: {}", filename)
    }
}

pub fn get_signed(filename: &str) -> Vec<i64> {
    if let Ok(lines) = read_lines(filename) {
        read_signed(lines)
    } else {
        panic!("Could not open file: {}", filename)
    }
}

pub fn get_unsigned(filename: &str) -> Vec<u64> {
    if let Ok(lines) = read_lines(filename) {
        read_unsigned(lines)
    } else {
        panic!("Could not open file: {}", filename)
    }
}

fn read_unsigned(lines: io::Lines<io::BufReader<File>>) -> Vec<u64> {
    // Consumes the iterator, returns an (Optional) String
    let mut numbers = Vec::new();
    for line in lines {
        if let Ok(ip) = line {
            let i = ip.parse::<u64>().unwrap();
            numbers.push(i);
        }
    }

    numbers
}

fn read_signed(lines: io::Lines<io::BufReader<File>>) -> Vec<i64> {
    // Consumes the iterator, returns an (Optional) String
    let mut numbers = Vec::new();
    for line in lines {
        if let Ok(ip) = line {
            let i = ip.parse::<i64>().unwrap();
            numbers.push(i);
        }
    }

    numbers
}

fn read_strings(lines: io::Lines<io::BufReader<File>>) -> Vec<String> {
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
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
