use std::cmp::max;
use std::collections::HashSet;
use std::io::prelude::*;
use std::io::{self};
use std::{fs::File, io::BufReader};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct PartNum {
    coords: (usize, usize),
    num: u32,
}
struct Matrix {
    // data: [[char; 140]; 140],
    data: Vec<Vec<char>>,
    symbols_list: Vec<(usize, usize)>,
}
impl Matrix {
    fn from_lines(lines: &Vec<String>) -> Matrix {
        let mut symbols_list = Vec::new();
        let mut data = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            let chars = line.chars().collect::<Vec<_>>();
            data.push(chars.clone());
            for (x, c) in chars.iter().enumerate() {
                if *c != '.' && !c.is_digit(10) {
                    println!("found {} at ({}, {})", *c, x, y);
                    symbols_list.push((x, y));
                }
                //     matrix.data[y][x] = *c;
            }
        }

        Matrix {
            // data: [['.'; 140]; 140],
            data,
            symbols_list,
        }
    }
    fn get(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.data.len() || y >= self.data.len() {
            return None;
        } else {
            Some(self.data[y][x])
        }
    }

    fn get_number_at(&self, x: i32, y: i32) -> Option<PartNum> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if self.get(x, y) == Some('.') {
            return None;
        }
        if self.get(x, y).is_none() {
            return None;
        }

        let mut n_chars: Vec<char> = Vec::new();
        // add current digit
        if let Some(c) = self.get(x, y) {
            if c.is_digit(10) {
                n_chars.push(c);
            } else {
                return None;
            }
        }

        // go back and check prev
        let mut start_x = x;
        loop {
            if start_x <= 0 {
                break;
            }
            let c = self.get(start_x - 1, y);
            if c == Some('.') || c == None {
                break;
            }
            let c = c.unwrap();
            if c.is_digit(10) == false {
                break;
            }

            n_chars.splice(0..0, [c].iter().cloned());
            start_x -= 1;
            // println!("{:#?}", n_chars.iter().collect::<String>());
        }

        let mut end_x = x;
        loop {
            if end_x >= 140 {
                break;
            }
            let c = self.get(end_x + 1, y);
            if c == None {
                break;
            }
            let c = c.unwrap();
            if c.is_digit(10) == false {
                break;
            }
            n_chars.push(c);
            end_x += 1;

            // println!("{:#?}", n_chars.iter().collect::<String>());
        }

        let n = match n_chars.iter().collect::<String>().parse::<u32>() {
            Ok(n) => n,
            Err(e) => {
                print!("{:#?} : {:#?}", e, n_chars.iter().collect::<String>());
                0
            }
        };

        Some(PartNum {
            coords: (start_x, y),
            num: n,
        })
    }

    fn sum_parts(&self) -> u64 {
        let mut read_coords: HashSet<(usize, usize)> = HashSet::new();
        let mut part_sum: u64 = 0;
        for symbol_coords in self.symbols_list.iter() {
            let mut mods = Vec::new();
            if symbol_coords.1 > 0 {
                // check row above
                mods.push((symbol_coords.0, symbol_coords.1 - 1));
                if symbol_coords.0 > 0 {
                    // check left
                    mods.push((symbol_coords.0 - 1, symbol_coords.1 - 1));
                }
                if symbol_coords.0 < 139 {
                    // check right
                    mods.push((symbol_coords.0 + 1, symbol_coords.1 - 1));
                }
            }

            if symbol_coords.1 < 139 {
                // check row below
                mods.push((symbol_coords.0, symbol_coords.1 + 1));
                if symbol_coords.0 > 0 {
                    mods.push((symbol_coords.0 - 1, symbol_coords.1 + 1));
                }
                if symbol_coords.0 < 139 {
                    mods.push((symbol_coords.0 + 1, symbol_coords.1 + 1));
                }
            }

            if symbol_coords.0 > 0 {
                // check left
                mods.push((symbol_coords.0 - 1, symbol_coords.1));
            }
            if symbol_coords.0 < 139 {
                // check right
                mods.push((symbol_coords.0 + 1, symbol_coords.1));
            }

            for m in mods {
                if let Some(part) = self.get_number_at(m.0 as i32, m.1 as i32) {
                    if !read_coords.contains(&(part.coords.0, part.coords.1)) {
                        part_sum += part.num as u64;
                        println!(
                            "part #{} at ({}, {})",
                            part.num, part.coords.0, part.coords.1
                        );
                    } else {
                        println!(
                            "part #{} at ({}, {}) already read. Not counting",
                            part.num, part.coords.0, part.coords.1
                        );
                    }
                    read_coords.insert((part.coords.0, part.coords.1));
                }
            }
        }
        part_sum
    }
}

fn main() {
    let input = read_input(3).unwrap();
    println!("star 1: {}", star1(input.to_owned()).unwrap());
    println!("star 2: {}", star2().unwrap());
}

fn read_input(day: u8) -> io::Result<Vec<String>> {
    let f = File::open(format!("input/{}.txt", day.to_string()))?;
    let f = BufReader::new(f);
    let mut lines = Vec::new();
    for line in f.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

fn star1(input: Vec<String>) -> anyhow::Result<u64> {
    let matrix = Matrix::from_lines(&input);
    let part_sum = matrix.sum_parts();

    Ok(part_sum)
}

fn star2() -> anyhow::Result<i32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_star1() {
        let input_lines: Vec<String> = INPUT.split("\n").map(|s| s.to_owned()).collect();
        let matrix = Matrix::from_lines(&input_lines);

        assert_eq!(matrix.data.len(), 10);
        assert_eq!(matrix.data[0][0], '4');
        assert_eq!(matrix.data[4][0], '6');

        let pn = PartNum {
            coords: (0, 0),
            num: 467,
        };
        assert_eq!(matrix.get_number_at(0, 0), Some(pn.clone()));
        assert_eq!(matrix.get_number_at(1, 0), Some(pn.clone()));
        assert_eq!(matrix.get_number_at(2, 0), Some(pn.clone()));

        assert_eq!(matrix.sum_parts(), 4361);
    }

    #[test]
    fn test_star2() {
        assert_eq!(super::star2().unwrap(), 0);
    }
}
