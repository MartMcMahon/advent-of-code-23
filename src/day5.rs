use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::{self};
use std::ops::Range;
use std::{fs::File, io::BufReader};

pub fn read_lines() -> io::Result<io::Lines<BufReader<File>>> {
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
    println!("star 2: {}", star2(lines));
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

fn get_seeds(s: String) -> Vec<u64> {
    s.split_whitespace()
        .skip(1)
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect()
}
fn get_seed_ranges(s: String) -> Vec<(u64, u64)> {
    let ns: Vec<u64> = s
        .split_whitespace()
        .skip(1)
        .map(|s| u64::from_str_radix(s, 10).unwrap())
        .collect();

    let mut res = Vec::new();
    for i in 0..ns.len() {
        if i % 2 == 0 {
            res.push((ns[i], ns[i + 1]));
        }
    }
    res
}

pub fn star1(lines: Vec<String>) -> u64 {
    let mut map_chain: Vec<Vec<GardenMap>> = Vec::new();
    let mut cats_list = lines.split(|s| s == "");
    let seeds_str: String = cats_list.nth(0).unwrap().to_vec()[0].to_owned();
    for cat_str in cats_list {
        let maps = GardenMap::vec_from_lines(cat_str.to_vec());
        map_chain.push(maps);
    }

    let mut locs = Vec::new();

    let seeds: Vec<u64> = get_seeds(seeds_str);
    for seed in seeds {
        let mut next_link = seed;
        for i in 0..7 {
            next_link = get_next_link(&map_chain[i], next_link);
        }
        locs.push(next_link);
    }

    locs.into_iter().reduce(|x, y| std::cmp::min(x, y)).unwrap()
}

// pub fn star2(lines: Vec<String>) -> u64 {
//     let mut cats_list = lines.split(|s| s == "");
//     // build map chain...
//     let mut map_chain: Vec<Vec<GardenMap>> = Vec::new();
//     let mut cats_list = lines.split(|s| s == "");
//     let seeds_str: String = cats_list.nth(0).unwrap().to_vec()[0].to_owned();
//     for cat_str in cats_list {
//         let maps = GardenMap::vec_from_lines(cat_str.to_vec());
//         map_chain.push(maps);
//     }

//     // ... but reverse it
//     map_chain.reverse();

//     // test potential seed
//     for pot_loc in 0..u64::max_value() {
//         for map in map_chain {

//         }

//     }
// }

fn star2(lines: Vec<String>) -> u64 {
    let mut map_chain: Vec<Vec<GardenMap>> = Vec::new();
    let mut cats_list = lines.split(|s| s == "");
    let seeds_str: String = cats_list.nth(0).unwrap().to_vec()[0].to_owned();
    for cat_str in cats_list {
        let maps = GardenMap::vec_from_lines(cat_str.to_vec());
        map_chain.push(maps);
    }

    let mut locs: Vec<u64> = Vec::new();

    let answer_map: HashMap<u64, u64> = HashMap::new();
    let seed_ranges: Vec<(u64, u64)> = get_seed_ranges(seeds_str);
    let mut c = 0;
    // for (start, length) in seed_ranges {
    //     let r = start..start + length;
    // r
    // println!("calcing seed range: {}", c);
    // for i in 0..7 {
    //     for map in map_chain[i] {
    //         if start < map.src_start
    //     }

    //     }
    // }

    // for seed in start..start + length {
    //     if let Some(l) = answer_map.get(&(seed)) {
    //         locs.push(*l);
    //         continue;
    //     }
    //     let mut next_link = seed;
    //     for i in 0..7 {
    //         next_link = get_next_link(&map_chain[i], next_link);
    //     }
    //     locs.push(next_link);
    // }
    // c += 1;
    // }
    // }

    // locs.into_iter().reduce(|x, y| std::cmp::min(x, y)).unwrap()
    0
}

Range {start: 50, end: 98}

Range {start: 52, end: 100}
79 14 55 13


enum GardenType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
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



        let garden_map: HashMap<GardenType, Vec<Range<u64>>> = HashMap::new();
        garden_map.insert(GardenType::SeedToSoil, Vec::new());
        garden_map.insert(GardenType::SoilToFertilizer, Vec::new());
        garden_map.insert(GardenType::FertilizerToWater, Vec::new());
        garden_map.insert(GardenType::WaterToLight, Vec::new());
        garden_map.insert(GardenType::LightToTemperature, Vec::new());
        garden_map.insert(GardenType::TemperatureToHumidity, Vec::new());
        garden_map.insert(GardenType::HumidityToLocation, Vec::new());

        garden_map[GardenType::SeedToSoil].push(Range {start: 50, end: 98});



        let mut maps = Vec::new();
        for line in lines.into_iter().skip(1) {
            let map_vals: Vec<u64> = line
                .split_whitespace()
                .map(|s| u64::from_str_radix(s, 10).unwrap())
                .collect();
            assert_eq!(map_vals.len(), 3);
            src_
            let range = Range {start: map_vals[1], end: map_vals[0] + map_vals[2]};
            maps.push(GardenMap {
                src_start: map_vals[1],
                dest_start: map_vals[0],
                length: map_vals[2],
            });
        }

        maps
    }

    // fn from_
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

    #[test]
    fn test_star2() {
        let lines = INPUT
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        assert_eq!(star2(lines.clone()), 46);
    }
}
