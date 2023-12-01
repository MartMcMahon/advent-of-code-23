use std::io::prelude::*;
use std::io::{self};
use std::{fs::File, io::BufReader};

fn main() {
    println!("star 1: {}", star1().unwrap());
    println!("star 2: {}", star2().unwrap());
}

fn star1() -> io::Result<i64> {
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

fn star2() -> anyhow::Result<u64> {
    let mut sum = 0;
    let f = File::open("input/1.txt")?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line?;
        // println!("line: {}", line);
        let first = first_num(&line);
        let last = last_num(&line);
        let line_val = first.to_string() + last.to_string().as_str();
        // println!("line_val: {}", line_val);
        assert_eq!(line_val.len(), 2);
        sum += u64::from_str_radix(&line_val, 10).unwrap();
        // println!("sum: {}", sum);
    }
    Ok(sum)
}

fn num_strings() -> Vec<&'static str> {
    vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
}

fn first_num(s: &str) -> i64 {
    let mut first_num = 0;
    let mut first_num_idx = s.len();
    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_digit() {
            first_num_idx = i;
            first_num = c.to_digit(10).unwrap() as i64;
            break;
        }
    }

    for (i, num_string) in num_strings().iter().enumerate() {
        let found_sub_str_idx = s.find(num_string);
        match found_sub_str_idx {
            Some(idx) => {
                if idx < first_num_idx {
                    first_num_idx = idx;
                    first_num = i as i64;
                }
            }
            None => {}
        }
    }

    first_num
}

fn last_num(s: &str) -> i64 {
    let mut last_num = 0;
    let mut last_num_idx = 0;

    for (i, c) in s.chars().enumerate() {
        if c.is_ascii_digit() {
            last_num_idx = i;
            last_num = c.to_digit(10).unwrap() as i64;
        }
    }

    for (i, num_string) in num_strings().iter().enumerate() {
        let matches: Vec<(usize, &str)> = s.match_indices(num_string).collect();
        if matches.len() > 0 {
            if matches[matches.len() - 1].0 >= last_num_idx {
                last_num_idx = matches[matches.len() - 1].0;
                last_num = i as i64;
            }
        }
    }

    last_num
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

#[cfg(test)]
#[test]
fn test_first_num() {
    let inputs = vec![
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];

    // "two1nine"
    assert_eq!(first_num(inputs[0]), 2);
    assert_eq!(last_num(inputs[0]), 9);

    // "eightwothree"
    assert_eq!(first_num(inputs[1]), 8);
    assert_eq!(last_num(inputs[1]), 3);

    // "abcone2threexyz"
    assert_eq!(first_num(inputs[2]), 1);
    assert_eq!(last_num(inputs[2]), 3);

    // "xtwone3four"
    assert_eq!(first_num(inputs[3]), 2);
    assert_eq!(last_num(inputs[3]), 4);

    // "4nineeightseven2"
    assert_eq!(first_digit(inputs[4]), '4');
    assert_eq!(first_num(inputs[4]), 4);
    assert_eq!(last_num(inputs[4]), 2);

    // "zoneight234"
    assert_eq!(first_num(inputs[5]), 1);
    assert_eq!(last_num(inputs[5]), 4);

    // "7pqrstsixteen"
    assert_eq!(first_num(inputs[6]), 7);
    assert_eq!(last_num(inputs[6]), 6);

    assert_eq!(first_num("nine"), 9);
    assert_eq!(last_num("nine"), 9);

    assert_eq!(first_num("xnine"), 9);
    assert_eq!(last_num("xnine"), 9);

    assert_eq!(first_num("twone"), 2);
    assert_eq!(last_num("twone"), 1);

    assert_eq!(first_num("six1"), 6);
    assert_eq!(last_num("six1"), 1);

    assert_eq!(first_num("eightwothree"), 8);
    assert_eq!(last_num("eightwothree"), 3);

    assert_eq!(first_num("eighthree"), 8);
    assert_eq!(last_num("eighthree"), 3);

    assert_eq!(first_num("sevenine"), 7);
    assert_eq!(last_num("sevenine"), 9);

    assert_eq!(first_num("oneeighttwo34dcjck5eightjznpzhxdlc"), 1);
    assert_eq!(last_num("oneeighttwo34dcjck5eightjznpzhxdlc"), 8);
}
