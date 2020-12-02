use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Based on https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

fn main() {
    day2_1();
}

fn day2_1() {
    if let Ok(lines) = read_lines("./data/input2") {
        let strings = read_strings(lines);
        for s in strings.iter() {
            let words = s.split(' ').collect::<Vec<_>>();
            let nums = words.get(0).unwrap().split('-');
            let char = words.get(1).unwrap().strip_suffix(':');
            let password = words.get(2).unwrap();
            let x = count_char_in(password, char.unwrap());
        }
    }
}

fn count_char_in(password: &str, char: &str) -> u32 {
    0
}

fn day1_1() {
    if let Ok(lines) = read_lines("./data/input1") {
        let numbers = read_numbers(lines);
        find_pair_result(numbers);
    }

}

fn day1_2() {
    if let Ok(lines) = read_lines("./data/input1") {
        let numbers = read_numbers(lines);
        find_trio_result(numbers);
    }

}

fn find_pair_result(numbers: Vec<i32>) {
    for x in numbers.iter() {
        for y in numbers.iter() {
            if x + y == 2020 {
                let a = x * y;
                println!("Answer is: {}", a);
                return;
            }
        }
    }
}

fn find_trio_result(numbers: Vec<i32>) {
    for x in numbers.iter() {
        for y in numbers.iter() {
            for z in numbers.iter() {
                if x + y + z == 2020 {
                    let a = x * y * z;
                    println!("Answer is: {}", a);
                    return;
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


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
