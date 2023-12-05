use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::io::{self};
use std::{fs::File, io::BufReader};

fn main() {
    let mut lines = Vec::new();
    for line in read_lines().unwrap() {
        let line = line.unwrap();
        lines.push(line);
    }
    println!("star 1: {}", star1(lines.clone()).unwrap());
    println!("star 2: {}", star2(lines).unwrap());
}

fn read_lines() -> io::Result<io::Lines<BufReader<File>>> {
    let f = File::open("input/4.txt")?;
    let f = BufReader::new(f);
    Ok(f.lines())
}

fn star1(lines: Vec<String>) -> anyhow::Result<u32> {
    let mut sum = 0;
    for line in lines {
        let pipe_index = line.find("|").unwrap();
        let col_index = line.find(":").unwrap();
        let (left, our_list) = line.split_at(pipe_index);
        let (game, win_list) = &left.split_at(col_index + 2);

        let re = Regex::new(r"^Card +(\d+): ").unwrap();
        let (full_cap, [num]) = re.captures(&game).unwrap().extract();
        let card_n = num.parse::<u32>().unwrap();

        let winning = win_list
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let ours = our_list
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let exp = count_winning_nums(winning, ours);
        if exp == 0 {
            continue;
        }
        sum += 1 << exp - 1;
    }

    Ok(sum)
}

fn count_winning_nums(winning: Vec<u32>, ours: Vec<u32>) -> u32 {
    let mut winning_set = HashSet::new();

    for x in winning.iter() {
        winning_set.insert(x);
    }

    let mut count = 0;
    for x in ours.iter() {
        if winning_set.contains(x) {
            count += 1;
        }
    }
    count
}

fn star2(lines: Vec<String>) -> anyhow::Result<i32> {
    let mut card_count = [1; 203];
    let mut card_tally = 0;

    for (i, line) in lines.into_iter().enumerate() {
        let pipe_index = line.find("|").unwrap();
        let col_index = line.find(":").unwrap();
        let (left, our_list) = line.split_at(pipe_index);
        let (game, win_list) = &left.split_at(col_index + 2);

        let re = Regex::new(r"^Card +(\d+): ").unwrap();
        let (full_cap, [num]) = re.captures(&game).unwrap().extract();
        let card_n = num.parse::<u32>().unwrap();

        let winning = win_list
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let ours = our_list
            .split_whitespace()
            .skip(1)
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        card_tally += card_count[i];
        let c = count_winning_nums(winning, ours);
        if c == 0 {
            continue;
        }
        for z in i + 1..i + 1 + c as usize {
            card_count[z] += card_count[i];
        }
    }

    // Ok(card_count.into_iter().sum())
    Ok(card_tally)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_star1() -> anyhow::Result<()> {
        let lines = INPUT
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        assert_eq!(star1(lines)?, 13);
        Ok(())
    }

    #[test]
    fn test_star2() -> anyhow::Result<()> {
        let lines = INPUT
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        assert_eq!(star2(lines)?, 30);
        Ok(())
    }
}
