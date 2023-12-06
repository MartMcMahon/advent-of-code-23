use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

fn read_lines() -> (Vec<String>, Vec<String>) {
    let f = File::open("input/6.txt").unwrap();
    let f = BufReader::new(f);
    let mut lines = Vec::new();
    for line in f.lines() {
        let line = line.unwrap();
        lines.push(line);
    }

    let times: Vec<String> = lines[0]
        .to_owned()
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect();
    let distances: Vec<String> = lines[1]
        .to_owned()
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect();
    (times, distances)
}

fn main() {
    let (times, distances) = read_lines();
    // println!(
    //     "star 1: {}",
    //     star1(times.clone(), distances.clone()).unwrap()
    // );

    let time_str = times.join("");
    let (_, time_str) = time_str.split_at(5);
    let distance_str = distances.join("");
    let (_, distance_str) = distance_str.split_at(9);

    let time = u128::from_str_radix(time_str, 10).unwrap();
    let distance = u128::from_str_radix(distance_str, 10).unwrap();

    println!("star 2: {}", star2(time, distance).unwrap());
}

fn star1(times: Vec<String>, distances: Vec<String>) -> anyhow::Result<i32> {
    let mut races: HashMap<i32, i32> = HashMap::new();
    for (i, time) in times.iter().enumerate().skip(1) {
        let distance = distances[i].parse::<i32>().unwrap();
        let time = time.parse::<i32>().unwrap();

        // println!("time: {}, dist: {}", time, distance);
        let avg_speed = distance / time;
        // println!("avg_speed: {}", avg_speed);
        let mut b = avg_speed;
        let mut c = 0;
        loop {
            if b <= 0 {
                break;
            }
            if (time - b) * b > distance {
                // println!("b: {}", b);
                c += 1;
            }
            b -= 1;
        }
        let mut b = avg_speed + 1;
        loop {
            if b >= time {
                break;
            }
            if (time - b) * b > distance {
                // println!("b: {}", b);
                c += 1;
            }
            b += 1;
        }

        races.insert(i as i32, c);
    }

    // println!("races: {:?}", races);

    let mut ans = 1;
    for (k, v) in races.iter() {
        // println!("{}: {}", k, v);
        ans *= v;
    }

    Ok(ans)
}

fn star2(time: u128, distance: u128) -> anyhow::Result<u128> {
    let avg_speed = distance / time;
    // println!("avg_speed: {}", avg_speed);
    let mut b = avg_speed;
    let mut c = 0;
    loop {
        if b <= 0 {
            break;
        }
        if (time - b) * b > distance {
            // println!("b: {}", b);
            c += 1;
        }
        b -= 1;
    }
    let mut b = avg_speed + 1;
    loop {
        if b >= time {
            break;
        }
        if (time - b) * b > distance {
            // println!("b: {}", b);
            c += 1;
        }
        b += 1;
    }

    Ok(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    fn test_input() -> (Vec<String>, Vec<String>) {
        let mut lines: Vec<String> = TEST_INPUT.split("\n").map(|s| s.to_owned()).collect();
        let times: Vec<String> = lines[0].split_whitespace().map(|s| s.to_owned()).collect();
        let distances: Vec<String> = lines[1].split_whitespace().map(|s| s.to_owned()).collect();
        (times, distances)
    }

    #[test]
    fn test_star1() -> anyhow::Result<()> {
        let (times, distances) = test_input();
        assert_eq!(star1(times, distances)?, 288);
        Ok(())
    }

    #[test]
    fn test_star2() -> anyhow::Result<()> {
        let (times, distances) = test_input();

        let time_str = times.join("");
        let (_, time_str) = time_str.split_at(5);
        let distance_str = distances.join("");
        let (_, distance_str) = distance_str.split_at(9);

        let time = u128::from_str_radix(time_str, 10).unwrap();
        let distance = u128::from_str_radix(distance_str, 10).unwrap();

        assert_eq!(star2(time, distance)?, 71503);

        Ok(())
    }
}
