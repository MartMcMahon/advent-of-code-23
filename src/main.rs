use std::io::prelude::*;
use std::io::{self};
use std::{fs::File, io::BufReader};

fn main() {
    println!("star 1: {}", star1().unwrap());
    println!("star 2: {}", star2().unwrap());
}

fn star1() -> anyhow::Result<i32> {
    let f = File::open("input/x.txt")?;
    let f = BufReader::new(f);

    Ok(0)
}

fn star2() -> anyhow::Result<i32> {
    Ok(0)
}
