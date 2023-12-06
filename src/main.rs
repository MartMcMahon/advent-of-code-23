use std::io::prelude::*;
use std::io::{self};
use std::{fs::File, io::BufReader};

fn read_lines() -> io::Result<io::Lines<BufReader<File>>> {
    let f = File::open("input/6.txt")?;
    let f = BufReader::new(f);
    Ok(f.lines())
}

fn main() {
    let mut lines = Vec::new();
    for line in read_lines().unwrap() {
        let line = line.unwrap();
        lines.push(line);
    }

    println!("star 1: {}", star1(lines.clone()).unwrap());
    println!("star 2: {}", star2(lines).unwrap());
}

fn star1(lines: Vec<String>) -> anyhow::Result<i32> {
    Ok(0)
}

fn star2(lines: Vec<String>) -> anyhow::Result<i32> {
    Ok(0)
}
