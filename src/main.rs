use std::io::prelude::*;
use std::io::{self};
use std::{fs::File, io::BufReader};
mod day6;

fn read_lines() -> io::Result<io::Lines<BufReader<File>>> {
    let f = File::open("input/x.txt")?;
    let f = BufReader::new(f);
    Ok(f.lines())
}

fn main() {
    let (time, distance) = day6::read_lines2();

    // println!("star 1: {}", star1(lines.clone()).unwrap());
    println!("star 2: {}", day6::star2(time, distance).unwrap());
}

fn star1(lines: Vec<String>) -> anyhow::Result<i32> {
    Ok(0)
}

fn star2(lines: Vec<String>) -> anyhow::Result<i32> {
    Ok(0)
}
