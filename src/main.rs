use regex::Regex;
use std::io::prelude::*;
use std::io::{self};
use std::{fs::File, io::BufReader};

fn read_lines() -> io::Result<io::Lines<BufReader<File>>> {
    let f = File::open("input/5.txt")?;
    let f = BufReader::new(f);
    Ok(f.lines())
}

fn main() {
    let mut lines = Vec::new();
    for line in read_lines().unwrap() {
        let line = line.unwrap();
        lines.push(line);
    }

    println!("star 1: {}", star1(lines.clone()));
    println!("star 2: {}", star2(lines).unwrap());
}

fn get_next_link(maps: &Vec<GardenMap>, i: u64) -> u64 {
    for map in maps {
        if i > map.src_start && i < map.src_start + map.length {
            // println!("following {} to {}", i, map.dest_start + i);
            return map.dest_start + i - map.src_start;
        }
    }
    i
}

fn star1(lines: Vec<String>) -> u64 {
    let mut map_chain: Vec<Vec<GardenMap>> = Vec::new();
    let mut cats_list = lines.split(|s| s == "");
    let seeds_str: String = cats_list.nth(0).unwrap().to_vec()[0].to_owned();
    for cat_str in cats_list {
        let maps = GardenMap::vec_from_lines(cat_str.to_vec());
        map_chain.push(maps);
    }

    let mut locs = Vec::new();

    let seeds: Vec<u64> = seeds_str
        .split_whitespace()
        .skip(1)
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect();
    for seed in seeds {
        let mut next_link = seed;
        for i in 0..7 {
            next_link = get_next_link(&map_chain[i], next_link);
        }
        locs.push(next_link);
    }

    locs.into_iter().reduce(|x, y| std::cmp::min(x, y)).unwrap()
}

fn star2(lines: Vec<String>) -> anyhow::Result<i32> {
    Ok(0)
}

struct GardenMap {
    src_start: u64,
    dest_start: u64,
    length: u64,
}
impl GardenMap {
    fn vec_from_lines(lines: Vec<String>) -> Vec<Self> {
        let re = Regex::new(r"^(\w+)-to-(\w+) map:").unwrap();
        let (_full_cap, [src, dest]) = re.captures(&lines[0]).unwrap().extract();

        println!("making map list for {}-to-{}", src, dest);

        let mut maps = Vec::new();
        for line in lines.into_iter().skip(1) {
            let map_vals: Vec<u64> = line
                .split_whitespace()
                .map(|s| u64::from_str_radix(s, 10).unwrap())
                .collect();
            assert_eq!(map_vals.len(), 3);
            maps.push(GardenMap {
                src_start: map_vals[1],
                dest_start: map_vals[0],
                length: map_vals[2],
            });
        }

        maps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_star1() {
        let lines = INPUT
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        assert_eq!(star1(lines.clone()), 35);
    }
}
