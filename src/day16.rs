use colored::Colorize;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::prelude::*;

pub fn read_input(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    buf
}

pub fn parse_grid(buf: Vec<u8>) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();
    let mut row = Vec::new();
    for c in buf {
        match c {
            b'\n' => {
                grid.push(row);
                row = Vec::new();
            }
            _ => row.push(c),
        }
    }
    grid
}

#[derive(Clone, Copy, Debug)]
struct Traveled {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}
impl Traveled {
    fn new() -> Traveled {
        Traveled {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Clone, Copy, Debug)]
struct Ray {
    pos: (usize, usize),
    dir: Dir,
}
impl Display for Ray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x, y) = self.pos;
        write!(f, "{:?} -> ({}, {})", self.dir, x, y,)
    }
}
impl Ray {
    fn proc(
        &mut self,
        grid: &Vec<Vec<u8>>,
        queue: &mut Vec<Ray>,
        energized: &mut Vec<Vec<Traveled>>,
    ) {
        let (x, y) = self.pos;
        let grid_spot = grid[y][x];
        let max_width = grid[0].len();
        let max_height = grid.len();
        match grid_spot {
            b'\\' => match self.dir {
                Dir::Up => {
                    self.dir = Dir::Left;
                    if self.step(energized, max_width, max_height) {
                        queue.push(*self);
                    }
                }
                Dir::Down => {
                    self.dir = Dir::Right;
                    if self.step(energized, max_width, max_height) {
                        queue.push(*self);
                    }
                }
                Dir::Left => {
                    self.dir = Dir::Up;
                    if self.step(energized, max_width, max_height) {
                        queue.push(*self);
                    }
                }
                Dir::Right => {
                    self.dir = Dir::Down;
                    if self.step(energized, max_width, max_height) {
                        queue.push(*self);
                    }
                }
            },
            b'/' => match self.dir {
                Dir::Up => {
                    self.dir = Dir::Right;
                    if self.step(energized, max_width, max_height) {
                        queue.push(*self);
                    }
                }
                Dir::Down => {
                    self.dir = Dir::Left;
                    if self.step(energized, max_width, max_height) {
                        queue.push(*self);
                    }
                }
                Dir::Left => {
                    self.dir = Dir::Down;
                    if self.step(energized, max_width, max_height) {
                        queue.push(*self);
                    }
                }
                Dir::Right => {
                    self.dir = Dir::Up;
                    if self.step(energized, max_width, max_height) {
                        queue.push(*self);
                    }
                }
            },
            b'|' => match self.dir {
                Dir::Left | Dir::Right => {
                    self.dir = Dir::Up;
                    if self.step(energized, max_width, max_height) {
                        queue.push(*self);
                    }

                    let mut new_ray = Ray {
                        pos: (x, y),
                        dir: Dir::Down,
                    };
                    if new_ray.step(energized, max_width, max_height) {
                        queue.push(new_ray);
                    }
                }
                _ => {
                    if self.step(energized, max_width, max_height) {
                        queue.push(*self);
                    }
                }
            },
            b'-' => match self.dir {
                Dir::Up | Dir::Down => {
                    self.dir = Dir::Left;
                    if self.step(energized, max_width, max_height) {
                        queue.push(*self);
                    }

                    let mut new_ray = Ray {
                        pos: (x, y),
                        dir: Dir::Right,
                    };
                    if new_ray.step(energized, max_width, max_height) {
                        queue.push(new_ray);
                    }
                }
                _ => {
                    if self.step(energized, max_width, max_height) {
                        queue.push(*self);
                    }
                }
            },

            b'.' => {
                if self.step(energized, max_width, max_height) {
                    queue.push(*self);
                }
            }
            _ => {
                panic!("bad char: {}", grid_spot)
            }
        }
    }

    fn step(
        &mut self,
        energized: &mut Vec<Vec<Traveled>>,
        max_width: usize,
        max_height: usize,
    ) -> bool {
        match self.dir {
            Dir::Up => {
                energized[self.pos.1][self.pos.0].up = true;
                if self.pos.1 == 0 {
                    return false;
                }
                self.pos.1 -= 1;
                return !energized[self.pos.1][self.pos.0].up;
            }
            Dir::Down => {
                energized[self.pos.1][self.pos.0].down = true;
                if self.pos.1 == max_height - 1 {
                    return false;
                }
                self.pos.1 += 1;
                return !energized[self.pos.1][self.pos.0].down;
            }
            Dir::Left => {
                energized[self.pos.1][self.pos.0].left = true;
                if self.pos.0 == 0 {
                    return false;
                }
                self.pos.0 -= 1;
                return !energized[self.pos.1][self.pos.0].left;
            }
            Dir::Right => {
                energized[self.pos.1][self.pos.0].right = true;
                if self.pos.0 == max_width - 1 {
                    return false;
                }
                self.pos.0 += 1;
                return !energized[self.pos.1][self.pos.0].right;
            }
        }
    }
}

pub fn star1(buf: Vec<u8>) -> i32 {
    let grid = parse_grid(buf);
    let mut queue = vec![Ray {
        pos: (0, 0),
        dir: Dir::Right,
    }];
    let mut energized = vec![vec![Traveled::new(); grid.len()]; grid.len()];
    energized[0][0].right = true;

    while queue.len() > 0 {
        let mut ray = queue.pop().unwrap();
        ray.proc(&grid, &mut queue, &mut energized);

        // let mut iy = 0;
        // for row in &energized {
        //     print!("\n");
        //     for (ix, x) in row.iter().enumerate() {
        //         if x.up {
        //             print!("{}", "^".red());
        //         } else if x.down {
        //             print!("{}", "v".red());
        //         } else if x.left {
        //             print!("{}", "<".red());
        //         } else if x.right {
        //             print!("{}", ">".red());
        //         } else {
        //             print!("{}", grid[iy][ix] as char);
        //         }
        //     }

        //     if iy < queue.len() {
        //         print!("                         {}", queue[iy]);
        //     }

        //     iy += 1;
        // }
        // print!("\ncount: {}\n", count_energized(&energized));
        // print!("\n");
        // std::io::stdin().read_line(&mut String::new()).unwrap();
        // std::thread::sleep(std::time::Duration::from_millis(100));

        // println!("{:?}", queue);
    }

    count_energized(&energized)
}

fn count_energized(energized: &Vec<Vec<Traveled>>) -> i32 {
    let mut count = 0;
    for row in energized {
        for x in row {
            if x.up || x.down || x.left || x.right {
                count += 1;
            }
        }
    }
    count
}

pub fn star2(buf: Vec<u8>) -> i32 {
    10
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

    // #[test]
    // fn test_star1() {
    //     let input = TEST_INPUT.as_bytes().to_vec();
    //     let x = star1(input);
    //     assert_eq!(x, 46);
    // }

    #[test]
    fn custom_test1() {
        let input = r#".|...
.....
.....
.-...
.....
"#;
        let x = star1(input.as_bytes().to_vec());
        assert_eq!(x, 9);

        let input = r#"|..
|..
|..
"#;
        let x = star1(input.as_bytes().to_vec());
        assert_eq!(x, 3);

        let input = r#"-...\
.....
.....
.....
\.../
"#;

        let x = star1(input.as_bytes().to_vec());
        assert_eq!(x, 16);

        let input = r#"-...\
.....
.....
|.-..
\.|./
"#;

        let x = star1(input.as_bytes().to_vec());
        assert_eq!(x, 19);

        let input = "/\n";
        let x = star1(input.as_bytes().to_vec());
        assert_eq!(x, 1);
    }

    #[test]
    fn test_step() {
        let mut grid = vec![vec![b'.'; 10]; 10];
        grid[3][4] = b'|';
        let mut energized = vec![vec![Traveled::new(); 10]; 10];
        let mut ray = Ray {
            pos: (3, 3),
            dir: Dir::Right,
        };
        let mut queue = vec![ray];
        while queue.len() > 0 {
            let mut ray = queue.pop().unwrap();
            ray.proc(&grid, &mut queue, &mut energized);
        }

        assert_eq!(energized[3][3].right, true);
        assert_eq!(energized[3][4].up, true);
        assert_eq!(energized[4][4].down, true);

        assert_eq!(energized[2][4].up, true);
        assert_eq!(energized[5][4].down, true);
    }

    #[test]
    fn test_star2() {
        let input = TEST_INPUT.as_bytes().to_vec();
        let x = star2(input);
        // assert_eq!(x, 10);
    }
}
