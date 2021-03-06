mod utils;
use lazy_static::lazy_static;

use crate::utils::{get_signed, get_strings, get_unsigned};
use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

fn main() {
    //let strings = get_strings("./data/input21");
    let strings = get_strings("./test/test21");
    let result = day21_1(&strings);
    println!("Result: {}", result);
    //let result = day14_2(&strings);
    //println!("Result: {}", result);
}

#[cfg(test)]
mod test21 {
    use super::*;
    static TEST_FILE: &str = "./test/test21";

    #[test]
    fn test21_1() {
        assert_eq!(day21_1(&get_strings(TEST_FILE)), 5);
    }
}

fn day21_1(strings: &Vec<String>) -> u64 {
    lazy_static! {
        static ref REING: Regex = Regex::new(r"([a-z ]*) \(contains ([a-z, ]*)\)").unwrap();
    }
    let mut i_map: HashMap<String, Ingredient> = HashMap::new();
    for s in strings {
        let cap = REING.captures(s);
        if cap.is_some() {
            let parts = cap.unwrap();
            let ingreds: Vec<&str> = parts[1].split(' ').collect();
            let allergens: Vec<&str> = parts[2].split(',').collect();
            for i in ingreds.iter() {
                if let Some(ing) = i_map.get_mut(*i) {
                    // Update existing ingredient in the map
                    for a in allergens.iter() {
                        if let Some(a_count) = ing.a_map.get_mut(a.trim()) {
                            // Allergen is already in map for this ingredient
                            *a_count = *a_count + 1;
                        } else {
                            // New allergen for this ingredient
                            ing.a_map.insert(a.trim().to_string(), 1);
                        }
                    }
                } else {
                    // Add new ingredient to the map
                    let mut a_map: HashMap<String, u64> = HashMap::new();
                    for a in allergens.iter() {
                        a_map.insert(a.trim().to_string(), 1);
                    }
                    i_map.insert(i.to_string(), Ingredient { a_map, count: 1 });
                }
            }
        } else {
            panic!("Could not parse line: {}", s);
        }
    }
    let mut result: u64 = 0;
    println!("The following ingredients contain no known allergens:");
    for (i, ing) in i_map.iter() {
        let mut confirmed = false;
        for (a, count) in ing.a_map.iter() {
            if *count > 1 {
                confirmed = true;
                break;
            }
        }
        if confirmed == false {
            println!("- {}", i);
            result = result + ing.count;
        }
    }
    result
}

struct Ingredient {
    a_map: HashMap<String, u64>,
    count: u64,
}

#[cfg(test)]
mod test14 {
    use super::*;
    static TEST_FILE: &str = "./test/test14";
    static TEST_FILE_2: &str = "./test/test14_2";

    #[test]
    fn test14_1() {
        assert_eq!(day14_1(&get_strings(TEST_FILE)), 165);
    }

    #[test]
    fn test14_2() {
        assert_eq!(day14_2(&get_strings(TEST_FILE_2)), 208);
    }
}

fn day14_1(strings: &Vec<String>) -> u64 {
    let mut mask = Bitmask {
        mask: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
    };
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for s in strings {
        let (label, value) = parse_line(s);
        if label == "mask" {
            mask = Bitmask {
                mask: value.to_string(),
            };
        } else {
            let (mem, val) = get_mem(label, value);
            let newvalue = mask.decode_value(val);
            memory.insert(mem, newvalue);
            //println!("mem[{}] = {}", mem, newvalue);
        }
    }
    let mut result: u64 = 0;
    for (_k, i) in memory {
        result = result + i;
    }
    result
}

fn day14_2(strings: &Vec<String>) -> u64 {
    let mut mask = Bitmask {
        mask: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
    };
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for s in strings {
        let (label, value) = parse_line(s);
        if label == "mask" {
            mask = Bitmask {
                mask: value.to_string(),
            };
        } else {
            let (mem, val) = get_mem(label, value);
            let addrs = mask.decode_address(mem);
            for a in addrs {
                memory.insert(a, val);
                //println!("a[0b{:08b} / {}] = {}", a, a, val);
            }
        }
    }
    let mut result: u64 = 0;
    for (_k, i) in memory {
        result = result + i;
    }
    result
}

fn parse_line(line: &str) -> (&str, &str) {
    let mut parts = line.split('=');
    if let Some(mut label) = parts.next() {
        label = label.trim();
        if let Some(mut value) = parts.next() {
            value = value.trim();
            return (label, value);
        }
    }
    panic!("Could not parse line: {}", line)
}

fn get_mem(label: &str, value: &str) -> (u64, u64) {
    lazy_static! {
        static ref REMEM: Regex = Regex::new(r"mem\[(\d+)\]").unwrap();
    }
    let cap = REMEM.captures(label);
    if cap.is_some() {
        if let Ok(mem) = cap.unwrap()[1].parse::<u64>() {
            if let Ok(val) = value.parse::<u64>() {
                return (mem, val);
            }
        }
    }
    panic!("Invalid mem= entry: {}", label)
}

struct Bitmask {
    mask: String,
}

impl Bitmask {
    fn decode_value(&self, value: u64) -> u64 {
        let mut newvalue = 0;
        let mut i: u64 = 1;
        for b in self.mask.chars().rev() {
            match b {
                'X' => newvalue = newvalue + (value & i),
                '1' => newvalue = newvalue + i,
                '0' => newvalue = newvalue,
                _ => panic!("Unexpected value in bitmask"),
            }
            i = i << 1;
        }

        newvalue
    }

    fn decode_address(&self, address: u64) -> Vec<u64> {
        //println!("Mask: {}", self.mask);
        //println!("Address before mask: 0b{:08b} / {}", address, address);
        let mut addrs: Vec<u64> = Vec::new();
        let mut i = 1;
        addrs.push(address);
        for b in self.mask.chars().rev() {
            let mut new_addrs: Vec<u64> = Vec::new();
            for a in addrs.iter_mut() {
                match b {
                    'X' => {
                        new_addrs.push(*a | i); // add new addr with bit set
                        *a &= !i; // unset bit in existing address
                    }
                    '1' => {
                        *a |= i;
                    }
                    '0' => {
                        // unchanged
                    }
                    _ => panic!("Unexpected value in bitmask"),
                }
            }
            for n in new_addrs {
                addrs.push(n)
            }
            i = i << 1;
        }
        addrs
    }
}

#[cfg(test)]
mod test13 {
    use super::*;
    static TEST_FILE: &str = "./test/test13";

    #[test]
    fn test13_1() {
        assert_eq!(day13_1(&get_strings(TEST_FILE)), 295);
    }
}

fn day13_1(strings: &Vec<String>) -> u64 {
    let mut result: u64 = 0;
    let mut earliest: u64 = u64::MAX;
    if let Ok(timestamp) = strings.get(0).unwrap().parse::<u64>() {
        let notes: Vec<&str> = strings.get(1).unwrap().split(',').collect();
        for bus_s in notes {
            if let Ok(bus) = bus_s.parse::<u64>() {
                let wait = bus - (timestamp % bus);
                if (wait < earliest) {
                    earliest = wait;
                    result = wait * bus;
                    println!("Bus: {} Wait:{}", bus, wait);
                }
            }
        }
    }
    result
}

fn day13_2(strings: &Vec<String>) -> u64 {
    let mut result: u64 = 0;
    let notes: Vec<&str> = strings.get(1).unwrap().split(',').collect();
    let mut targets: Vec<(u64, u64)> = Vec::new();
    let mut count = 0;
    let mut first = 0;
    let mut high = 0;
    for bus_s in notes {
        if let Ok(bus) = bus_s.parse::<u64>() {
            if first == 0 {
                first = bus;
            }
            targets.push((bus, count));
            high = max(high, bus);
            println!("Bus:{} Seq:{}", bus, count);
        } else {
            if bus_s == "x" {
                // skip timeslot
            } else {
                println!("Invalid note: {}", bus_s);
            }
        }
        count = count + 1;
    }

    let mut found = false;
    let mut test: u64 = 0;
    while found == false {
        test = test + first;
        //println!("{}", test);
        for (bus, seq) in &targets {
            found = test % *bus == *seq;
            if found == false {
                break;
            }
        }
    }
    test
}

#[cfg(test)]
mod test12 {
    use super::*;
    static TEST_FILE: &str = "./test/test12";

    #[test]
    fn test12_1() {
        assert_eq!(day12_1(&get_strings(TEST_FILE)), 25);
    }

    #[test]
    fn test12_2() {
        assert_eq!(day12_2(&get_strings(TEST_FILE)), 286);
    }
}

const NORTH: u16 = 0;
const EAST: u16 = 90;
const SOUTH: u16 = 180;
const WEST: u16 = 270;

struct Waypoint {
    east: i64,
    north: i64,
    location: Location,
}

impl Waypoint {
    fn north(&mut self, value: u16) {
        self.north = self.north + value as i64;
    }
    fn south(&mut self, value: u16) {
        self.north = self.north - value as i64;
    }
    fn east(&mut self, value: u16) {
        self.east = self.east + value as i64;
    }
    fn west(&mut self, value: u16) {
        self.east = self.east - value as i64;
    }
    fn left(&mut self, value: u16) {
        // rotate waypoint counter-clockwise 'value' degrees
        for _r in 0..value / 90 {
            let north = self.north;
            let east = self.east;
            self.east = 0 - north;
            self.north = east;
        }
    }
    fn right(&mut self, value: u16) {
        // rotate waypoint clockwise 'value' degrees
        for _r in 0..value / 90 {
            let north = self.north;
            let east = self.east;
            self.east = north;
            self.north = 0 - east;
        }
    }
    fn forward(&mut self, value: u16) {
        for _i in 0..value {
            self.location.north = self.location.north + self.north;
            self.location.east = self.location.east + self.east;
        }
    }
    fn manhattan(&self) -> u64 {
        println!(
            "Location: {:?} Waypoint: {}, {}",
            self.location, self.east, self.north
        );
        self.location.manhattan()
    }
}

#[derive(Debug)]
struct Location {
    heading: u16,
    east: i64,
    north: i64,
}

impl Location {
    fn north(&mut self, value: u16) {
        self.north = self.north + value as i64;
    }
    fn south(&mut self, value: u16) {
        self.north = self.north - value as i64;
    }
    fn east(&mut self, value: u16) {
        self.east = self.east + value as i64;
    }
    fn west(&mut self, value: u16) {
        self.east = self.east - value as i64;
    }
    fn left(&mut self, value: u16) {
        let heading: i32 = self.heading as i32 - value as i32;
        if heading < 0 {
            self.heading = (heading + 360) as u16;
        } else {
            self.heading = heading as u16;
        }
    }
    fn right(&mut self, value: u16) {
        self.heading = self.heading + value;
        if self.heading >= 360 {
            self.heading = self.heading - 360;
        }
    }
    fn forward(&mut self, value: u16) {
        match self.heading {
            NORTH => self.north(value),
            EAST => self.east(value),
            SOUTH => self.south(value),
            WEST => self.west(value),
            _ => panic!("Invalid heading!"),
        }
    }
    fn manhattan(&self) -> u64 {
        println!("N{} E{} Heading: {}", self.north, self.east, self.heading);
        (self.north.abs() + self.east.abs()) as u64
    }
}

fn day12_1(strings: &Vec<String>) -> u64 {
    let mut location = Location {
        heading: EAST,
        east: 0,
        north: 0,
    };
    for i in strings {
        let (act, val_s) = i.split_at(1);
        if let Ok(val) = val_s.parse::<u16>() {
            match act {
                "N" => location.north(val),
                "S" => location.south(val),
                "E" => location.east(val),
                "W" => location.west(val),
                "L" => location.left(val),
                "R" => location.right(val),
                "F" => location.forward(val),
                _ => panic!("Invalid action!"),
            }
        }
        location.manhattan();
    }
    location.manhattan()
}

fn day12_2(strings: &Vec<String>) -> u64 {
    let location = Location {
        heading: EAST,
        east: 0,
        north: 0,
    };
    let mut waypoint = Waypoint {
        east: 10,
        north: 1,
        location,
    };
    for i in strings {
        let (act, val_s) = i.split_at(1);
        if let Ok(val) = val_s.parse::<u16>() {
            match act {
                "N" => waypoint.north(val),
                "S" => waypoint.south(val),
                "E" => waypoint.east(val),
                "W" => waypoint.west(val),
                "L" => waypoint.left(val),
                "R" => waypoint.right(val),
                "F" => waypoint.forward(val),
                _ => panic!("Invalid action!"),
            }
        }
        waypoint.manhattan();
    }
    waypoint.manhattan()
}

#[cfg(test)]
mod test11 {
    use super::*;
    static TEST_FILE: &str = "./test/test11";
    static TEST_FILE_R1: &str = "./test/test11_r1";
    static TEST_FILE_R2: &str = "./test/test11_r2";
    static TEST_FILE_R3: &str = "./test/test11_r3";

    #[test]
    fn test11_1() {
        assert_eq!(day11_1(&get_strings(TEST_FILE)), 37);
    }

    #[test]
    fn test11_2() {
        assert_eq!(day11_2(&get_strings(TEST_FILE)), 26);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SeatMap {
    seats: HashMap<(u8, u8), char>,
    rows: u8,
    cols: u8,
    ranged: bool,
    max_occup: u8,
    max_iter: u16,
}

impl SeatMap {
    fn new(strings: &Vec<String>) -> SeatMap {
        let mut seats: HashMap<(u8, u8), char> = HashMap::new();
        let mut row = 0;
        let mut col = 0;
        for line in strings {
            col = 0;
            for seat in line.chars() {
                seats.insert((row, col), seat);
                col = col + 1;
            }
            row = row + 1;
        }
        SeatMap {
            seats,
            rows: row,
            cols: col,
            ranged: false,
            max_occup: 4,
            max_iter: 999,
        }
    }

    fn count_occupied(&self) -> u64 {
        let mut count = 0;
        for ((_row, _col), seat) in &self.seats {
            if *seat == '#' {
                count = count + 1;
            }
        }
        count
    }

    fn seat_passengers(&mut self) -> bool {
        let mut new_seats: HashMap<(u8, u8), char> = HashMap::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let seat = self.seats.get(&(row, col)).unwrap();
                let mut new_seat = *seat;
                if *seat == 'L' {
                    if self.check_seats(row, col) == 0 {
                        new_seat = '#';
                    }
                } else if *seat == '#' {
                    if self.check_seats(row, col) >= self.max_occup {
                        new_seat = 'L';
                    }
                }
                new_seats.insert((row, col), new_seat);
            }
        }
        let stable = new_seats == self.seats;
        self.seats = new_seats;
        stable
    }

    const COORDS: [(i16, i16); 8] = [
        (-1, 0),
        (-1, 1),
        (-1, -1),
        (0, 1),
        (0, -1),
        (1, 0),
        (1, 1),
        (1, -1),
    ];

    fn check_seats(&self, row: u8, col: u8) -> u8 {
        let mut count = 0;
        //println!("Checking seat {},{}", row, col);
        for (r, c) in SeatMap::COORDS.iter() {
            let mut testrow = row as i16;
            let mut testcol = col as i16;
            let mut searching = true;
            while searching && count < self.max_occup {
                searching = self.ranged;
                testrow = testrow + r;
                testcol = testcol + c;
                if testrow >= 0 && testcol >= 0 {
                    if let Some(seat) = self.seats.get(&(testrow as u8, testcol as u8)) {
                        //println!("  - {},{}: {}", rtest, ctest, *seat);
                        if *seat == '#' {
                            count = count + 1;
                            // stop searching on an occupied seat
                            searching = false;
                        } else if self.ranged && *seat == 'L' {
                            // stop searching on an empty seat
                            searching = false;
                        }
                    } else {
                        // stop searching if seat is out of bounds
                        searching = false;
                    }
                } else {
                    // stop searching if seat is out of bounds
                    searching = false;
                }
            }
        }
        //println!("({},{}) : {}", row, col, count);
        count
    }

    fn print(&self) {
        for row in 0..self.rows {
            let mut line = String::new();
            for col in 0..self.cols {
                let seat = self.seats.get(&(row, col));
                line.push(*seat.unwrap());
            }
            println!("{}", line);
        }
        println!("++++ Ends ++++");
    }
}

fn day11_1(strings: &Vec<String>) -> u64 {
    let mut map = SeatMap::new(strings);
    let mut stable = false;
    println!("Searching...");
    while (stable == false) {
        stable = map.seat_passengers();
        //map.print();
    }
    map.count_occupied()
}

fn day11_2(strings: &Vec<String>) -> u64 {
    let mut map = SeatMap::new(strings);
    map.max_occup = 5;
    map.ranged = true;
    println!("Searching...");
    let mut stable = false;
    let mut count = 0;
    while (stable == false) {
        stable = map.seat_passengers();
        count = count + 1;
    }
    //map.print();
    map.count_occupied()
}

#[cfg(test)]
mod test10 {
    use super::*;
    static TEST_FILE: &str = "./test/test10";
    static TEST_FILE_2: &str = "./test/test10_2";

    #[test]
    fn test10_1() {
        assert_eq!(day10_1(&get_unsigned(TEST_FILE_2)), 220);
    }

    #[test]
    fn test10_2_1() {
        assert_eq!(day10_2(&get_unsigned(TEST_FILE)), 8);
    }

    #[test]
    fn test10_2_2() {
        assert_eq!(day10_2(&get_unsigned(TEST_FILE_2)), 19208);
    }
}

fn day10_1(numbers: &Vec<u64>) -> u64 {
    let mut adaptors = numbers.clone();
    adaptors.sort();
    let mut count1 = 0;
    let mut count3 = 1; // Last diff is always 3
    let mut prev: u64 = 0;
    for a in adaptors {
        let diff = a - prev;
        match diff {
            1 => count1 = count1 + 1,
            3 => count3 = count3 + 1,
            _ => println!("Diff = {}", diff),
        }
        prev = a;
    }
    println!(
        "There are {} 1-jolt and {} 3-jolt differences",
        count1, count3
    );
    count3 * count1
}

fn day10_2(numbers: &Vec<u64>) -> u64 {
    let trib = trib(32);
    let mut adaptors = numbers.clone();
    adaptors.sort();
    if let Some(high) = adaptors.last() {
        adaptors.push(*high + 3);
    } else {
        panic!("Could not get last adaptor!")
    }
    let mut routes: u64 = 1;
    let mut b = 0;
    let mut seq = 0;
    for a in adaptors {
        if a - b == 1 {
            seq = seq + 1;
        } else if a - b == 3 {
            routes = routes * *trib.get(seq).unwrap() as u64;
            seq = 0;
        } else {
            panic!("Difference of {}!", a - b);
        }
        b = a;
    }

    routes
}

fn trib(max: u32) -> Vec<u32> {
    let mut seq: Vec<u32> = Vec::new();
    seq.push(1);
    for _t in 0..max {
        let mut value: u32 = 0;
        let mut first: i64 = seq.len() as i64 - 3;
        if first < 0 {
            first = 0;
        }
        if let Some(prev) = seq.get(first as usize..) {
            for x in prev {
                value = value + *x as u32;
            }
        }
        seq.push(value);
    }
    seq
}

#[cfg(test)]
mod test09 {
    use super::*;
    static TEST_FILE: &str = "./test/test9";

    #[test]
    fn test09_1() {
        assert_eq!(day9_1(5, &get_unsigned(TEST_FILE)), 127);
    }

    #[test]
    fn test09_2() {
        assert_eq!(day9_2(127, &get_unsigned(TEST_FILE)), 62);
    }
}

fn day9_1(preamble: u16, numbers: &Vec<u64>) -> u64 {
    let mut buffer: Vec<u64> = Vec::new();
    let mut counter = 0;
    for n in numbers {
        if counter < preamble {
            counter = counter + 1;
            buffer.push(*n);
        } else {
            if !checksum(*n, &buffer) {
                return *n;
            }
            buffer.remove(0);
            buffer.push(*n);
        }
    }
    panic!("Not found")
}

fn day9_2(invalid: u64, numbers: &Vec<u64>) -> u64 {
    let mut searching = true;
    let mut bottom = 0;
    let mut top = 1;
    while bottom < numbers.len() {
        let range = numbers.get(bottom..top);
        let sum = sum_range(range);
        if sum == invalid {
            return min_range(range) + max_range(range);
        } else if sum > invalid {
            bottom = bottom + 1;
            top = bottom + 1;
        } else {
            top = top + 1;
        }
    }
    panic!("Encryption weakness not found!")
}

fn max_range(range: Option<&[u64]>) -> u64 {
    if range.is_some() {
        match range.unwrap().iter().max() {
            Some(max) => return *max,
            None => panic!("Range for max_range() is empty"),
        }
    }
    panic!("Range for max_range() is invalid")
}

fn min_range(range: Option<&[u64]>) -> u64 {
    let mut min: u64 = 0;
    if range.is_some() {
        match range.unwrap().iter().min() {
            Some(min) => return *min,
            None => panic!("Range for min_range() is empty"),
        }
    }
    panic!("Range for min_range() is invalid")
}

fn sum_range(range: Option<&[u64]>) -> u64 {
    let mut sum = 0;
    if range.is_some() {
        for i in range.unwrap().iter() {
            sum = sum + i;
        }
    }
    sum
}

fn checksum(target: u64, buffer: &Vec<u64>) -> bool {
    for a in buffer {
        for b in buffer {
            if a != b {
                //println!("Test: {}+{}=={}", a, b, target);
                if a + b == target {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod test08 {
    use super::*;
    static TEST_FILE: &str = "./test/test8";

    #[test]
    fn test08_1() {
        assert_eq!(day8_1(&get_strings(TEST_FILE)), 5);
    }

    #[test]
    fn test08_2() {
        assert_eq!(day8_2(&get_strings(TEST_FILE)), 8);
    }
}

#[derive(Clone, PartialEq, Eq)]
enum VmOp {
    Invalid,
    Nop,
    Acc,
    Jmp,
}

#[derive(Clone)]
struct Instruction {
    op: VmOp,
    val: i32,
}

#[derive(Clone)]
struct VirtualMachine {
    code: Vec<Instruction>,
    pc: u32,
    acc: i64,
    trace: HashSet<u32>,
    run: bool,
    debug: bool,
}

impl VirtualMachine {
    fn new(strings: &Vec<String>, debug: bool) -> VirtualMachine {
        let mut code: Vec<Instruction> = Vec::new();
        let trace: HashSet<u32> = HashSet::new();
        let mut i = 0;
        for s in strings {
            let words: Vec<&str> = s.split(' ').collect();
            if let Some(op_s) = words.get(0) {
                if let Some(val_s) = words.get(1) {
                    if let Ok(val) = val_s.parse::<i32>() {
                        let op;
                        match *op_s {
                            "nop" => op = VmOp::Nop,
                            "acc" => op = VmOp::Acc,
                            "jmp" => op = VmOp::Jmp,
                            _ => op = VmOp::Invalid,
                        }
                        if debug {
                            println!("Instruction {}: {} {}", i, op_s, val);
                        }
                        code.push(Instruction { op, val });
                        i = i + 1;
                    } else {
                        println!("{} could not be parsed as i32", val_s);
                    }
                } else {
                    println!("Could locate val in {}", s);
                }
            } else {
                println!("Could locate op in {}", s);
            }
        }
        VirtualMachine {
            code,
            pc: 0,
            acc: 0,
            trace,
            run: false,
            debug,
        }
    }

    fn run_to_repeat(&mut self) -> i64 {
        self.trace.clear();
        self.pc = 0;
        self.acc = 0;
        self.run = true;
        while (self.run) {
            if self.trace.contains(&self.pc) {
                self.run = false;
                println!("Repeat at {}", self.pc);
            } else {
                self.trace.insert(self.pc);
                if self.debug {
                    print!("{} -> ", self.pc);
                }
                if let Some(i) = self.code.get(self.pc as usize) {
                    match i.op {
                        VmOp::Nop => self.nop(i.val),
                        VmOp::Acc => self.acc(i.val),
                        VmOp::Jmp => self.jmp(i.val),
                        VmOp::Invalid => self.err(),
                    }
                    if self.normal_exit() {
                        self.run = false;
                    }
                }
            }
        }
        self.acc
    }

    fn nop(&mut self, val: i32) {
        // Do nothing
        self.pc = self.pc + 1;
    }

    fn acc(&mut self, val: i32) {
        self.acc = self.acc + val as i64;
        self.pc = self.pc + 1;
    }

    fn jmp(&mut self, val: i32) {
        let pc = self.pc as i64 + val as i64;
        if pc < 0 {
            println!("Error: jmp to invalid instruction at {}", self.pc);
            self.run = false;
        } else {
            self.pc = pc as u32;
        }
    }

    fn err(&mut self) {
        println!("Error: invalid op code at {}", self.pc);
        self.run = false;
    }

    fn normal_exit(&self) -> bool {
        self.pc == self.code.len() as u32
    }

    fn switch(&mut self, i: usize) {
        if let Some(mut it) = self.code.get_mut(i) {
            if it.op == VmOp::Jmp {
                it.op = VmOp::Nop;
            } else if it.op == VmOp::Nop {
                it.op = VmOp::Jmp;
            } else {
                panic!("Invalid switch operation!")
            }
        }
    }
}

fn day8_1(strings: &Vec<String>) -> i64 {
    VirtualMachine::new(strings, false).run_to_repeat()
}

fn day8_2(strings: &Vec<String>) -> i64 {
    let acc = 0;
    let vm = VirtualMachine::new(strings, false);
    let mut dvm = vm.clone();
    let mut i = 0;
    for mut it in vm.code {
        if it.op == VmOp::Jmp || it.op == VmOp::Nop {
            dvm.switch(i);
            dvm.run_to_repeat();
            dvm.switch(i);
            if dvm.normal_exit() {
                println!("Normal end!");
                return dvm.acc;
            }
        }
        i = i + 1;
    }
    acc
}

#[cfg(test)]
mod test07 {
    use super::*;
    static TEST_FILE: &str = "./test/test7";
    static TEST_FILE_2: &str = "./test/test7_2";

    #[test]
    fn test07_1() {
        assert_eq!(day7_1(&get_strings(TEST_FILE)), 4);
    }

    #[test]
    fn test07_2() {
        assert_eq!(day7_2(&get_strings(TEST_FILE_2)), 126);
    }
}

pub struct BagRule {
    bag: String,
    contents: Vec<(u8, String)>,
}

fn get_outer_bags<'a>(target: &str, rules: &'a HashMap<String, Vec<(u8, String)>>) -> Vec<&'a str> {
    let mut bags: Vec<&str> = Vec::new();
    for (bag, r) in rules {
        for (num, name) in r {
            if name == target {
                bags.push(&bag);
            }
        }
    }
    bags
}

fn get_inner_bags<'a>(target: &str, rules: &'a HashMap<String, Vec<(u8, String)>>) -> u64 {
    let mut count = 0;
    if let Some(bag) = rules.get(target) {
        println!("{} bags contain:", target);
        for (num, name) in bag {
            println!("   - {} {} bags", num, name);
            let contents = get_inner_bags(name, rules);
            println!("      - which contain {} other bags", contents);
            let m: u64 = *num as u64;
            count = count + (contents * m) + m;
            println!("      - making {} accumulated bags", count);
        }
    }
    println!("{} bags contain {} other bags", target, count);
    count
}

fn get_bag_rules(strings: &Vec<String>) -> HashMap<String, Vec<(u8, String)>> {
    let mut rules: HashMap<String, Vec<(u8, String)>> = HashMap::new();
    lazy_static! {
        static ref REBAGHD: Regex = Regex::new(r"([a-z]*\s[a-z]*)\sbags\scontain").unwrap();
        static ref REBAGIT: Regex = Regex::new(r"(\d+)\s([a-z]*\s[a-z]*)\sbags?[.,]").unwrap();
    }
    for s in strings.iter() {
        let head = REBAGHD.captures(s);
        if head.is_some() {
            let bag = head.unwrap()[1].to_string();
            let mut contents: Vec<(u8, String)> = Vec::new();
            //println!("{} bags contain:", bag);
            for item in REBAGIT.captures_iter(s) {
                let num = item[1].parse::<u8>().unwrap();
                let nam = item[2].to_string();
                //println!("- {} {} bags", num, nam);
                contents.push((num, nam));
            }
            rules.insert(bag, contents);
        }
    }
    rules
}

fn day7_2(strings: &Vec<String>) -> u64 {
    let mut rules: HashMap<String, Vec<(u8, String)>> = get_bag_rules(strings);
    let mut bagset: HashSet<String> = HashSet::new();
    let count = get_inner_bags("shiny gold", &rules);
    count
}

fn day7_1(strings: &Vec<String>) -> u32 {
    let mut rules: HashMap<String, Vec<(u8, String)>> = get_bag_rules(strings);
    let mut bagset: HashSet<String> = HashSet::new();
    let bags = get_outer_bags("shiny gold", &rules);
    println!("First pass:");
    for b in bags.iter() {
        println!("- found: {}", b);
        bagset.insert(b.to_string());
    }

    let mut growing = true;
    let mut count = 0;
    while growing {
        let mut bags = Vec::new();
        for b in bagset.iter() {
            let more = get_outer_bags(b, &rules);
            for m in more.iter() {
                println!("- found: {}", m);
                bags.push(m.to_string());
            }
        }
        for b in bags.iter() {
            bagset.insert(b.to_string());
        }
        if count == bagset.len() {
            // exit loop if we haven't found any more viable outer bags
            growing = false;
        } else {
            count = bagset.len();
            println!("recursing");
        }
    }

    bagset.len() as u32
}
// ===================================================================================
#[cfg(test)]
mod test06 {
    use super::*;
    static TEST_FILE: &str = "./test/test6";

    #[test]
    fn test06_1() {
        assert_eq!(day6_1(&get_strings(TEST_FILE)), 11);
    }

    #[test]
    fn test06_2() {
        assert_eq!(day6_2(&get_strings(TEST_FILE)), 6);
    }
}

struct AnswerGroup {
    answers: HashMap<char, u16>,
    size: u16,
}

impl AnswerGroup {
    fn new() -> AnswerGroup {
        let answers: HashMap<char, u16> = HashMap::new();
        AnswerGroup { answers, size: 0 }
    }
}

fn get_answer_groups(strings: &Vec<String>) -> Vec<AnswerGroup> {
    let mut groups: Vec<AnswerGroup> = Vec::new();
    let mut group = AnswerGroup::new();
    for s in strings.iter() {
        if s.len() == 0 {
            // start a new group
            groups.push(group);
            group = AnswerGroup::new();
        } else {
            group.size = group.size + 1;
            for c in s.chars() {
                if let Some(count) = group.answers.get_mut(&c) {
                    *count = *count + 1;
                } else {
                    group.answers.insert(c, 1);
                }
            }
        }
    }
    // Save final group
    if group.size > 0 {
        groups.push(group);
    }
    groups
}

fn day6_1(strings: &Vec<String>) -> u32 {
    let groups = get_answer_groups(strings);
    let mut sum = 0;
    let mut group = 0;
    for g in groups.iter() {
        let mut count = 0;
        for (_q, a) in g.answers.iter() {
            if *a > 0 {
                count = count + 1;
            }
        }
        //println!("Group {} count: {}", group, count);
        group = group + 1;
        sum = sum + count;
    }

    sum
}

fn day6_2(strings: &Vec<String>) -> u32 {
    let groups = get_answer_groups(strings);
    let mut sum = 0;
    let mut group = 0;
    for g in groups.iter() {
        let mut count = 0;
        for (_q, a) in g.answers.iter() {
            if *a == g.size {
                count = count + 1;
            }
        }
        //println!("Group {} count: {}", group, count);
        group = group + 1;
        sum = sum + count;
    }

    sum
}

#[cfg(test)]
mod test05 {
    use super::*;
    static TEST_FILE: &str = "./test/test5";

    #[test]
    fn test05_1() {
        assert_eq!(day5_1(&get_strings(TEST_FILE)), 820);
    }
}

fn day5_1(strings: &Vec<String>) -> u32 {
    let seats = get_seats(strings);
    let mut high = 0;
    for seat in seats.iter() {
        high = max(high, *seat);
    }
    high
}

fn day5_2(strings: &Vec<String>) -> u32 {
    let mut seats = get_seats(strings);
    seats.sort();
    let mut prev = 0;
    let mut target = 0;
    for seat in seats.iter() {
        println!("Seat: {}", seat);
        if seat - prev == 2 {
            println!("My seat?: {}", prev + 1);
            target = prev + 1;
        }
        prev = *seat;
    }
    target
}

fn get_seats(strings: &Vec<String>) -> Vec<u32> {
    let mut seats: Vec<u32> = Vec::new();
    for s in strings.iter() {
        let mut row = (0, 127, 128);
        let mut col = (0, 7, 8);
        for c in s.chars() {
            match c {
                'B' => row = upper(row),
                'F' => row = lower(row),
                'L' => col = lower(col),
                'R' => col = upper(col),
                _ => println!("Invalid character"),
            }
            //println!("col: {:?} row: {:?}", col, row);
        }
        let seat: u32 = ((row.0 * 8) + col.0) as u32;
        //println!("c`ode: {} - col: {} row: {} seat: {}", s, col.0, row.0, seat);
        seats.push(seat);
    }
    seats
}

fn lower(range: (u16, u16, u16)) -> (u16, u16, u16) {
    (range.0, range.1 - (range.2 / 2), range.2 / 2)
}

fn upper(range: (u16, u16, u16)) -> (u16, u16, u16) {
    (range.0 + (range.2 / 2), range.1, range.2 / 2)
}

#[cfg(test)]
mod test04 {
    use super::*;
    static TEST_FILE: &str = "./test/test4";
    static TEST_FILE_INVALID: &str = "./test/test4_invalid";
    static TEST_FILE_VALID: &str = "./test/test4_valid";

    #[test]
    fn test04_1() {
        assert_eq!(day4_1(&get_strings(TEST_FILE)), 2);
    }

    #[test]
    fn test04_2_invalid() {
        assert_eq!(day4_2(&get_strings(TEST_FILE_INVALID)), 0);
    }

    #[test]
    fn test04_2_valid() {
        assert_eq!(day4_2(&get_strings(TEST_FILE_VALID)), 3);
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

    fn validate_fields(&self) -> bool {
        self.check_fields()
            && self.check_byr()
            && self.check_iyr()
            && self.check_eyr()
            && self.check_hgt()
            && self.check_hcl()
            && self.check_ecl()
            && self.check_pid()
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
        let mut valid = false;
        if self.fields.contains_key(field) {
            let value = self.fields.get(field).unwrap();
            let year = check_num(value, 4);
            match year {
                Ok(y) => valid = y >= min && y <= max,
                Err(e) => println!("{}", e),
            }
        }
        valid
    }

    fn check_hgt(&self) -> bool {
        if self.fields.contains_key("hgt") {
            let mut valid = false;
            let value: &str = self.fields.get("hgt").unwrap();
            lazy_static! {
                static ref REHGT: Regex = Regex::new(r"(\d*)([ci][mn])").unwrap();
            }

            let caps = REHGT.captures(value);
            if caps.is_some() {
                let height = caps.unwrap();
                let htype = height.get(2).map_or("", |m| m.as_str());
                let hnum = height.get(1).map_or("", |m| m.as_str());
                match htype {
                    "cm" => valid = check_num_range(hnum, 150, 193),
                    "in" => valid = check_num_range(hnum, 59, 76),
                    _ => println!("Unknown heigth type: {}", htype),
                }
            }
            if !valid {
                println!("Height invalid: {}", value);
            }
            return valid;
        } else {
            println!("Height missing");
            return false;
        }
    }

    fn check_hcl(&self) -> bool {
        if self.fields.contains_key("hcl") {
            let value: &str = self.fields.get("hcl").unwrap();
            lazy_static! {
                static ref REHCL: Regex = Regex::new(r"[#][0-9a-f]{6}").unwrap();
            }

            let caps = REHCL.captures(value);
            if caps.is_some() {
                return true;
            } else {
                println!("Hair colour invalid: {}", value);
                return false;
            }
        } else {
            println!("Hair colour missing");
            return false;
        }
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

    fn check_pid(&self) -> bool {
        if self.fields.contains_key("pid") {
            let value: &str = self.fields.get("pid").unwrap();
            return check_num(value, 9).is_ok();
        }
        false
    }
}

fn check_num_range(value: &str, min: u64, max: u64) -> bool {
    let num = value.parse::<u64>();
    if num.is_ok() {
        let num = num.unwrap();
        return num >= min && num <= max;
    }
    false
}

fn check_num(value: &str, required_len: usize) -> Result<u64, String> {
    if value.len() == required_len {
        let num = value.parse::<u64>();
        if num.is_ok() {
            return Ok(num.unwrap());
        } else {
            return Err(format!("Could not parse as u64: {}", value));
        }
    }
    Err(format!("Wrong length: {}", value))
}

fn read_passports(strings: &Vec<String>) -> Vec<PassportEntry> {
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
    entries
}

fn day4_1(strings: &Vec<String>) -> u32 {
    let mut valid = 0;
    let entries = read_passports(strings);
    for p in entries {
        if p.check_fields() {
            valid = valid + 1;
        }
    }

    valid
}

fn day4_2(strings: &Vec<String>) -> u32 {
    let mut valid = 0;
    let entries = read_passports(strings);
    for p in entries {
        if p.validate_fields() {
            valid = valid + 1;
        }
    }

    valid
}

#[cfg(test)]
mod test03 {
    use super::*;
    static TEST_FILE: &str = "./test/test3";

    #[test]
    fn test03_1() {
        assert_eq!(day3_1(&get_strings(TEST_FILE)), 7);
    }

    #[test]
    fn test03_2() {
        assert_eq!(day3_2(&get_strings(TEST_FILE), &day3_rules()), 336);
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
mod test02 {
    use super::*;
    static TEST_FILE: &str = "./test/test2";

    #[test]
    fn test02_1() {
        assert_eq!(day2_1(&get_strings(TEST_FILE)), 2);
    }

    #[test]
    fn test02_2() {
        assert_eq!(day2_2(&get_strings(TEST_FILE)), 1);
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
mod test01 {
    use super::*;
    static TEST_FILE: &str = "./test/test1";

    #[test]
    fn test01_1() {
        assert_eq!(day1_1(get_signed(TEST_FILE)), 514579);
    }

    #[test]
    fn test01_2() {
        assert_eq!(day1_2(get_signed(TEST_FILE)), 241861950);
    }
}

fn day1_1(numbers: Vec<i64>) -> u64 {
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

fn day1_2(numbers: Vec<i64>) -> u64 {
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
