use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Based on https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./data/input1") {
        let numbers = read_numbers(lines);
        for x in numbers.iter() {
            for y in numbers.iter() {
                for z in numbers.iter() {
                    if x + y + z == 2020 {
                        let a = x * y * z;
                        println!("Answer is: {}", a);
                    }
                }
            }
        }
    }
}

fn read_numbers(lines: io::Lines<io::BufReader<File>>) -> Vec<i32> {
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

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
