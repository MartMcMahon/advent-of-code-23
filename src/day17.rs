use colored::*;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

pub fn read_input(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    buf
}

pub fn parse_grid(buf: Vec<u8>) -> Vec<Vec<Node>> {
    let mut grid = Vec::new();
    let mut row = Vec::new();
    let mut y = 0;
    let mut x = 0;
    for c in buf {
        match c {
            b'\n' => {
                grid.push(row);
                row = Vec::new();
                x = 0;
                y += 1;
            }
            _ => {
                row.push(Node::new(
                    x,
                    y,
                    0,
                    i32::from_str_radix(&format!("{}", c as char), 10).unwrap(),
                ));
                x += 1;
            }
        }
    }

    let grid_size = grid.len();
    for row in grid.iter_mut() {
        for node in row.iter_mut() {
            node.distance = node.distance(grid_size as i32 - 1, grid_size as i32 - 1);
        }
    }
    grid
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug)]
struct Node {
    x: i32,
    y: i32,
    cost: i32,
    distance: i32,
    distance_traveled: i32,
    total_heat_loss: i32,
    slc: VecDeque<Dir>,
    heat_loss: i32,
    dest: bool,
    path: Vec<(i32, i32)>,
}
impl Node {
    fn new(x: i32, y: i32, distance: i32, heat_loss: i32) -> Node {
        Node {
            x,
            y,
            cost: i32::max_value(),
            distance,
            heat_loss,
            distance_traveled: 0,
            total_heat_loss: heat_loss,
            slc: VecDeque::new(),
            dest: false,
            path: Vec::new(),
        }
    }
    fn distance(&self, x: i32, y: i32) -> i32 {
        (self.x - x).abs() + (self.y - y).abs()
    }
    fn neighbors_coords(&self, max_val: i32) -> Vec<(i32, i32, Dir)> {
        let mut neighbors = Vec::new();
        if self.x > 0 {
            neighbors.push((self.x - 1, self.y, Dir::West));
        }
        if self.x < max_val - 1 {
            neighbors.push((self.x + 1, self.y, Dir::East));
        }
        if self.y > 0 {
            neighbors.push((self.x, self.y - 1, Dir::North));
        }
        if self.y < max_val - 1 {
            neighbors.push((self.x, self.y + 1, Dir::South));
        }

        neighbors
    }
}

pub fn star1(buf: Vec<u8>) -> i32 {
    let mut grid = parse_grid(buf);
    let mut priority_queue: VecDeque<Node> = VecDeque::new();
    let grid_size = grid.len();
    grid[grid_size - 1][grid_size - 1].dest = true;
    // costs nothing to get to the start
    grid[0][0].total_heat_loss = 0;
    grid[0][0].cost = 0;
    priority_queue.push_back(grid[0][0].clone());

    let mut traveled = Vec::new();

    let mut ans = i32::max_value();
    while !priority_queue.is_empty() {
        let active = priority_queue.pop_front().unwrap();
        if active.dest {
            ans = active.total_heat_loss;
            break;
        }
        // check neighbors
        let neighbors = active.neighbors_coords(grid.len() as i32);
        for neigh_coords in neighbors {
            let n = &mut grid[neigh_coords.1 as usize][neigh_coords.0 as usize];

            if traveled.contains(&(neigh_coords.0, neigh_coords.1)) {
                continue;
            }

            if active.slc.len() == 3 && active.slc.iter().all(|d| *d == neigh_coords.2) {
                continue;
            }
            match active.slc.back() {
                Some(Dir::North) => {
                    if neigh_coords.2 == Dir::South {
                        continue;
                    }
                }
                Some(Dir::South) => {
                    if neigh_coords.2 == Dir::North {
                        continue;
                    }
                }
                Some(Dir::East) => {
                    if neigh_coords.2 == Dir::West {
                        continue;
                    }
                }
                Some(Dir::West) => {
                    if neigh_coords.2 == Dir::East {
                        continue;
                    }
                }
                _ => {}
            }

            n.slc = active.slc.clone();
            n.slc.push_back(neigh_coords.2);
            while n.slc.len() > 3 {
                n.slc.pop_front();
            }

            n.total_heat_loss = active.total_heat_loss + n.heat_loss;
            n.distance_traveled = active.distance_traveled + 1;
            n.cost = n.total_heat_loss + n.distance + n.distance_traveled;
            // if active.path.contains(&(active.x, active.y)) {
            //     n.cost *= 2;
            // }

            n.path = active.path.clone();
            n.path.push((active.x, active.y));

            priority_queue.push_back(n.clone());

            priority_queue
                .make_contiguous()
                .sort_by(|a, b| a.cost.cmp(&b.cost));
        }

        traveled.push((active.x, active.y));

        // for (y, row) in grid.iter().enumerate() {
        //     for (x, node) in row.iter().enumerate() {
        //         if priority_queue[0].path.contains(&(x as i32, y as i32)) {
        //             let c = node.cost.to_string();
        //             print!("{:3} ", c.red());
        //         } else {
        //             print!("{:3} ", node.heat_loss.to_string().green());
        //         }
        //     }

        //     print!("\n");
        // }
        // print!("\n\n");
        // std::thread::sleep(std::time::Duration::from_millis(200));
    }

    ans
}

pub fn star2(buf: Vec<u8>) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"#;

    #[test]
    fn test_star1() {
        let input = TEST_INPUT.as_bytes().to_vec();
        let x = star1(input);
        assert_eq!(x, 102);
    }

    #[test]
    fn test_star2() {
        let input = TEST_INPUT.as_bytes().to_vec();
        let x = star2(input);
        // assert_eq!(x, 10);
    }
}
