use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{self};
use std::{fs::File, io::BufReader};

#[derive(Debug)]
#[repr(C)]
enum Dir {
    Left,
    Right,
}
impl Dir {
    fn from_byte(b: u8) -> Self {
        match b {
            b'L' => Dir::Left,
            b'R' => Dir::Right,
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(Debug, Hash)]
struct Address<'a> {
    next: (&'a str, &'a str),
}

pub fn read_file() -> io::Result<io::Lines<BufReader<File>>> {
    let f = File::open("input/8.txt")?;
    let f = BufReader::new(f);
    Ok(f.lines())
}

fn parse_dirs_line(line: &str) -> Vec<Dir> {
    let mut dirs = Vec::new();
    for b in line.as_bytes() {
        dirs.push(Dir::from_byte(*b));
    }
    dirs
}

pub fn main() {
    let mut lines = Vec::new();
    for line in read_file().unwrap() {
        let line = line.unwrap();
        lines.push(line);
    }

    println!("star 1: {:#?}", star1(lines.clone()).unwrap());
    println!("star 2: {:#?}", star2(lines.clone()).unwrap());
}

fn maps<'a>(lines: &'a [String]) -> HashMap<String, Address<'a>> {
    let mut addrs: HashMap<String, Address> = HashMap::new();
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    for line in lines {
        let (_full_cap, [this, left, right]) = re.captures(line).unwrap().extract();

        addrs.insert(
            this.to_owned(),
            Address {
                next: (left, right),
            },
        );
    }
    addrs
}

fn maps_with_starting<'a>(lines: &'a [String]) -> (HashMap<String, Address<'a>>, Vec<String>) {
    let mut addrs: HashMap<String, Address> = HashMap::new();
    let mut starting_addrs = Vec::new();
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let is_starting_re = Regex::new(r"(..A)").unwrap();
    for line in lines {
        let (_full_cap, [this, left, right]) = re.captures(line).unwrap().extract();
        addrs.insert(
            this.to_owned(),
            Address {
                next: (left, right),
            },
        );

        if is_starting_re.is_match(this) {
            starting_addrs.push(this.to_owned());
        }
    }

    (addrs, starting_addrs)
}

#[derive(Debug)]
struct Addr {
    left: usize,
    right: usize,
    is_end: bool,
}

fn star2_map<'a>(lines: &'a [String]) -> (Vec<Addr>, Vec<usize>) {
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let mut str_vec = Vec::new();
    let mut str_to_idx_map = HashMap::new();

    lines[2..].iter().enumerate().for_each(|(idx, line)| {
        let (_full_cap, [this, left, right]) = re.captures(line).unwrap().extract();
        str_to_idx_map.insert(this, idx);
        str_vec.push((this, left, right));
    });

    let mut map = Vec::new();
    let mut start_idxs = Vec::new();
    let start_re = Regex::new(r"..A").unwrap();
    let end_re = Regex::new(r"..Z").unwrap();
    for (this, left, right) in str_vec.iter() {
        let mut is_end = false;
        if start_re.is_match(this) {
            start_idxs.push(map.len());
        } else if end_re.is_match(this) {
            is_end = true;
        }
        map.push(Addr {
            left: *str_to_idx_map.get(left).unwrap(),
            right: *str_to_idx_map.get(right).unwrap(),
            is_end,
        });
    }
    (map, start_idxs)
}

pub fn star1(lines: Vec<String>) -> anyhow::Result<i32> {
    let dirs = parse_dirs_line(&lines[0]);
    let addrs = maps(&lines[2..]);
    let mut this = addrs.get("AAA").unwrap();
    let mut steps = 0;
    loop {
        let mut next_str = "";
        match dirs[steps % dirs.len()] {
            Dir::Left => next_str = this.next.0,
            Dir::Right => next_str = this.next.1,
        }
        if next_str == "ZZZ" {
            break;
        }
        this = addrs.get(next_str).unwrap();
        steps += 1;
    }
    steps += 1;
    Ok(steps as i32)
}

pub fn star2(lines: Vec<String>) -> anyhow::Result<i32> {
    let dirs = parse_dirs_line(&lines[0]);
    let (map, mut starting_idxs) = star2_map(&lines[..]);
    println!("built map: {:#?}", map);
    let mut end_count = 0;
    map.iter().for_each(|a| {
        if a.is_end {
            end_count += 1;
        }
    });

    println!("starting: {}", starting_idxs.len());
    println!("ends: {}", end_count);
    let mut steps = 0;

    loop {
        for i in 0..starting_idxs.len() {
            starting_idxs[i] = match dirs[steps % dirs.len()] {
                Dir::Left => map[starting_idxs[i]].left,
                Dir::Right => map[starting_idxs[i]].right,
            }
        }

        steps += 1;
        if starting_idxs.iter().all(|p| map[*p].is_end) {
            return Ok(steps as i32);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    #[test]
    fn test_star1() {
        let input = INPUT.lines().map(|s| s.to_owned()).collect();
        assert_eq!(star1(input).unwrap(), 2)
    }

    #[test]
    fn test_star2() {
        let star2_test_input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
        let input = star2_test_input.lines().map(|s| s.to_owned()).collect();

        assert_eq!(star2(input).unwrap(), 6)
    }
}
