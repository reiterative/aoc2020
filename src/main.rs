mod utils;

use crate::utils::{read_lines, read_numbers, read_strings};
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    if let Ok(lines) = read_lines("./data/input4") {
        let strings = read_strings(lines);
        let valid = day4_1(&strings);
        println!("Valid passports: {}", valid);
    }
}

#[cfg(test)]
mod test4 {
    use super::*;
    static TEST_FILE: &str = "./test/test4";

    #[test]
    fn test4_1() {
        let valid;
        if let Ok(lines) = read_lines(TEST_FILE) {
            let strings = read_strings(lines);
            valid = day4_1(&strings);
        } else {
            panic!("Missing test file: {}", TEST_FILE)
        }
        assert_eq!(valid, 2);
    }
}

struct PassportEntry {
    fields: HashMap<String, String>,
}

impl PassportEntry {
    fn required_fields() -> Vec<(&'static str, bool)> {
        let fields = vec![
            ("byr", false),
            ("iyr", false),
            ("eyr", false),
            ("hgt", false),
            ("hcl", false),
            ("ecl", false),
            ("pid", false),
            ("cid", true),
        ];
        fields
    }

    fn check_fields(&self) -> bool {
        for (name, optional) in PassportEntry::required_fields().iter() {
            if !self.fields.contains_key(*name) {
                if !optional {
                    println!("Missing: {}", name);
                    return false;
                }
            }
        }
        true
    }

    fn validate(&self) -> bool {
        self.check_byr() && self.check_iyr() && self.check_eyr()
    }

    fn check_byr(&self) -> bool {
        self.check_year_range("byr", 1920, 2002)
    }

    fn check_iyr(&self) -> bool {
        self.check_year_range("iyr", 2010, 2020)
    }

    fn check_eyr(&self) -> bool {
        self.check_year_range("eyr", 2020, 2030)
    }

    fn check_year_range(&self, field: &str, min: u64, max: u64) -> bool {
        if self.fields.contains_key(field) {
            let value = self.fields.get(field).unwrap();
            let year = check_num(value, 4);
            match year {
                Some(y) => return y >= min && y <= max,
                None => return false,
            }
        }
        false
    }

    fn check_ecl(&self) -> bool {
        if self.fields.contains_key("ecl") {
            let value: &str = self.fields.get("ecl").unwrap();
            match value {
                "amb" => return true,
                "blu" => return true,
                "brn" => return true,
                "gry" => return true,
                "grn" => return true,
                "hzl" => return true,
                "oth" => return true,
                &_ => return false,
            }
        }
        false
    }
}

fn check_num(value: &str, required_len: usize) -> Option<u64> {
    if value.len() == required_len {
        let num = value.parse::<u64>();
        if num.is_ok() {
            return Some(num.unwrap());
        }
    }
    None
}

fn day4_1(strings: &Vec<String>) -> u32 {
    let mut valid = 0;
    let mut entries: Vec<PassportEntry> = Vec::new();
    let mut fields = HashMap::new();
    for s in strings {
        if s.len() == 0 {
            entries.push(PassportEntry {
                fields: fields.clone(),
            });
            fields.clear();
        } else {
            let words = s.split(' ').collect::<Vec<_>>();
            for w in words {
                let field_iter = w.split(':');
                if let Some((field, value)) = field_iter.collect_tuple() {
                    fields.insert(field.to_string(), value.to_string());
                }
            }
        }
    }

    for p in entries {
        if p.check_fields() {
            valid = valid + 1;
        }
    }

    valid
}

#[cfg(test)]
mod test3 {
    use super::*;
    static TEST_FILE: &str = "./test/test3";

    #[test]
    fn test3_1() {
        let mut valid = 0;
        if let Ok(lines) = read_lines(TEST_FILE) {
            let strings = read_strings(lines);
            valid = day3_1(&strings);
        } else {
            panic!("Missing test file: {}", TEST_FILE)
        }
        assert_eq!(valid, 7);
    }

    #[test]
    fn test3_2() {
        let mut valid = 0;
        if let Ok(lines) = read_lines(TEST_FILE) {
            let strings = read_strings(lines);
            valid = day3_2(&strings, &day3_rules());
        } else {
            panic!("Missing test file: {}", TEST_FILE)
        }
        assert_eq!(valid, 336);
    }
}

fn day3_rules() -> Vec<(i32, i32)> {
    let mut rules = Vec::new();
    rules.push((1, 1));
    rules.push((3, 1));
    rules.push((5, 1));
    rules.push((7, 1));
    rules.push((1, 2));

    rules
}

fn day3_2(strings: &Vec<String>, rules: &Vec<(i32, i32)>) -> u64 {
    let mut total: u64 = 1;
    for r in rules.iter() {
        let mut xpos = 0;
        let mut ypos = 0;
        let mut ytgt = 0;
        let mut trees = 0;
        for s in strings.iter() {
            //println!("len: {} x: {} y: {} ytgt: {}", s.len(), xpos, ypos, ytgt);
            if ytgt == ypos {
                //println!("Checking");
                if check_for_trees(&xpos, s) {
                    trees = trees + 1;
                }
                xpos = xpos + r.0;
                ytgt = ytgt + r.1;
            }
            ypos = ypos + 1;
        }
        println!("Rule R:{},D:{} - trees hit: {}", r.0, r.1, trees);
        total = total * trees;
    }
    println!("Total: {}", total);

    total
}

fn day3_1(strings: &Vec<String>) -> u32 {
    let mut xpos = 0;
    let mut ypos = 0;
    let mut trees: u32 = 0;
    for s in strings.iter() {
        //println!("len: {} x: {} y: {}", s.len(), xpos, ypos);
        if check_for_trees(&xpos, s) {
            trees = trees + 1;
        }
        xpos = xpos + 3;
        ypos = ypos + 1;
    }
    println!("Trees hit: {}", trees);

    trees
}

static TREE: &'static str = "#";
fn check_for_trees(xpos: &i32, map: &str) -> bool {
    let len = map.len();
    let mut x = *xpos as usize;
    while x >= len {
        x = x - len;
    }
    map[x..x + 1].eq(TREE)
}

#[cfg(test)]
mod test2 {
    use super::*;
    static TEST_FILE: &str = "./test/test2";

    #[test]
    fn test2_1() {
        let mut valid = 0;
        if let Ok(lines) = read_lines(TEST_FILE) {
            let strings = read_strings(lines);
            valid = day2_1(&strings);
        } else {
            panic!("Missing test file: {}", TEST_FILE)
        }
        assert_eq!(valid, 2);
    }

    #[test]
    fn test2_2() {
        let mut valid = 0;
        if let Ok(lines) = read_lines(TEST_FILE) {
            let strings = read_strings(lines);
            valid = day2_2(&strings);
        } else {
            panic!("Missing test file: {}", TEST_FILE)
        }
        assert_eq!(valid, 1);
    }
}

fn day2_1(strings: &Vec<String>) -> u32 {
    let mut valid = 0;
    for s in strings.iter() {
        let words = s.split(' ').collect::<Vec<_>>();
        let nums = words.get(0).unwrap().split('-').collect::<Vec<_>>();
        let min = nums.get(0).unwrap().parse::<u32>().unwrap();
        let max = nums.get(1).unwrap().parse::<u32>().unwrap();
        let target = words.get(1).unwrap().strip_suffix(':').unwrap();
        let password = words.get(2).unwrap();
        let matches = password.matches(target).count() as u32;
        if matches < min || matches > max {
            println!("Fail: {} - {} is > {} or < {} ", s, matches, max, min);
        } else {
            valid = valid + 1;
        }
    }
    println!("Total valid: {}", valid);

    valid
}

fn day2_2(strings: &Vec<String>) -> u32 {
    let mut valid = 0;
    for s in strings.iter() {
        let words = s.split(' ').collect::<Vec<_>>();
        let nums = words.get(0).unwrap().split('-').collect::<Vec<_>>();
        let first = nums.get(0).unwrap().parse::<usize>().unwrap();
        let second = nums.get(1).unwrap().parse::<usize>().unwrap();
        let target = words.get(1).unwrap().strip_suffix(':').unwrap();
        let password = words.get(2).unwrap();
        let mut count = 0;
        if password[first - 1..first] == *target {
            count = count + 1;
        }
        if password[second - 1..second] == *target {
            count = count + 1;
        }
        if count == 1 {
            valid = valid + 1;
        } else {
            println!("Fail: {}", s);
        }
    }
    println!("Total valid: {}", valid);

    valid
}

#[cfg(test)]
mod test1 {
    use super::*;
    static TEST_FILE: &str = "./test/test1";

    #[test]
    fn test1_1() {
        let mut valid = 0;
        if let Ok(lines) = read_lines(TEST_FILE) {
            let numbers = read_numbers(lines);
            valid = day1_1(numbers);
        }
        assert_eq!(valid, 514579);
    }

    #[test]
    fn test1_2() {
        let mut valid = 0;
        if let Ok(lines) = read_lines(TEST_FILE) {
            let numbers = read_numbers(lines);
            valid = day1_2(numbers);
        }
        assert_eq!(valid, 241861950);
    }
}

fn day1_1(numbers: Vec<i32>) -> u64 {
    let mut a: u64 = 0;
    for x in numbers.iter() {
        for y in numbers.iter() {
            if x + y == 2020 {
                a = (x * y) as u64;
                println!("Answer is: {}", a);
                return a;
            }
        }
    }
    a
}

fn day1_2(numbers: Vec<i32>) -> u64 {
    let mut a: u64 = 0;
    for x in numbers.iter() {
        for y in numbers.iter() {
            for z in numbers.iter() {
                if x + y + z == 2020 {
                    a = (x * y * z) as u64;
                    println!("Answer is: {}", a);
                    return a;
                }
            }
        }
    }
    a
}
