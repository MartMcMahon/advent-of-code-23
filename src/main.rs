use std::io::prelude::*;
use std::io::{self};
use std::{fs::File, io::BufReader};

fn main() {
    let problem1 = problem1().unwrap();
    println!("Problem 1: {}", problem1);
}

fn problem1() -> io::Result<i64> {
    let mut sum: i64 = 0;

    let f = File::open("input/1.txt")?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line?;
        let first = first_digit(&line);
        let last = last_digit(&line);
        let line_val = first.to_string() + last.to_string().as_str();
        sum += i64::from_str_radix(&line_val, 10).unwrap();
    }

    Ok(sum)
}

fn first_digit(s: &str) -> char {
    for c in s.chars() {
        if c.is_ascii_digit() {
            return c;
        }
    }
    panic!("No digit found in {}", s);
}

fn last_digit(s: &str) -> char {
    for c in s.chars().rev() {
        if c.is_ascii_digit() {
            return c;
        }
    }
    panic!("No digit found in {}", s);
}
