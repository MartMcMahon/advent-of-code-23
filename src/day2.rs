use regex::Regex;
use std::io::prelude::*;
use std::{fs::File, io::BufReader};

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}
#[derive(Debug, PartialEq)]
struct Round {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}
impl Round {
    fn pow(&self) -> u32 {
        let mut pow = 0;
        let red = match self.red {
            Some(r) => r,
            None => 0,
        };
        let green = match self.green {
            Some(g) => g,
            None => 0,
        };
        let blue = match self.blue {
            Some(b) => b,
            None => 0,
        };
        red * green * blue
    }
}
impl Game {
    fn new() -> Self {
        Self {
            id: 0,
            rounds: Vec::new(),
        }
    }
    fn is_valid(&self) -> bool {
        // {red: only 12 red cubes, 13 green cubes, and 14 blue cubes}
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;

        for round in &self.rounds {
            if let Some(red) = round.red {
                if red > max_red {
                    return false;
                }
            }

            if let Some(green) = round.green {
                if green > max_green {
                    return false;
                }
            }

            if let Some(blue) = round.blue {
                if blue > max_blue {
                    return false;
                }
            }
        }
        return true;
    }
    fn get_min_cubes(&self) -> Round {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for round in &self.rounds {
            if let Some(r) = round.red {
                if r > min_red {
                    min_red = r;
                }
            }
            if let Some(g) = round.green {
                if g > min_green {
                    min_green = g;
                }
            }
            if let Some(b) = round.blue {
                if b > min_blue {
                    min_blue = b;
                }
            }
        }

        Round {
            red: Some(min_red),
            green: Some(min_green),
            blue: Some(min_blue),
        }
    }
}

fn main() {
    println!("star 1: {}", star1().unwrap());
    println!("star 2: {}", star2().unwrap());
}

fn star1() -> anyhow::Result<u32> {
    let f = File::open("input/2.txt")?;
    let f = BufReader::new(f);

    let mut valid_games_sum = 0;
    for line in f.lines() {
        let line = line?;
        let game = proc_line(line);
        if game.is_valid() {
            valid_games_sum += game.id
        }
    }

    Ok(valid_games_sum)
}

fn proc_line(line: String) -> Game {
    let split1 = line.split(":").collect::<Vec<&str>>();
    let game_split = split1[0].split(" ").collect::<Vec<&str>>();
    let id = game_split[1].parse::<u32>().unwrap();

    let mut game = Game {
        id,
        rounds: Vec::new(),
    };
    let split2 = split1[1].split(";").collect::<Vec<&str>>();
    for s in split2 {
        let mut round = Round {
            red: None,
            green: None,
            blue: None,
        };
        let re = Regex::new(r"(\d+) (\w+)").unwrap();
        for (_full_cap, [num, color]) in re.captures_iter(s).map(|c| c.extract()) {
            let n = num.parse::<u32>().unwrap();
            match color {
                "red" => round.red = Some(n),
                "green" => round.green = Some(n),
                "blue" => round.blue = Some(n),
                _ => panic!("unknown color"),
            }
        }

        game.rounds.push(round);
    }
    game
}

fn star2() -> anyhow::Result<i32> {
    let f = File::open("input/2.txt")?;
    let f = BufReader::new(f);

    let mut game_pos_sum = 0;
    for line in f.lines() {
        let line = line?;
        let game = proc_line(line);
        let min_set: Round = game.get_min_cubes();
        let pow = min_set.pow();
        game_pos_sum += pow as i32;
    }
    Ok(game_pos_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_proc_line() {
        assert_eq!(
            proc_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()),
            Game {
                id: 1,
                rounds: vec![
                    Round {
                        red: Some(4),
                        green: None,
                        blue: Some(3),
                    },
                    Round {
                        red: Some(1),
                        green: Some(2),
                        blue: Some(6),
                    },
                    Round {
                        red: None,
                        green: Some(2),
                        blue: None,
                    },
                ],
            }
        );
    }

    #[test]
    fn test_star2() {
        let test_line_1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = proc_line(test_line_1.to_string());
        let min_set: Round = game.get_min_cubes();
        let pow = min_set.pow();
        assert_eq!(pow, 48);

        let test_line_2 = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let game = proc_line(test_line_2.to_string());
        let min_set: Round = game.get_min_cubes();
        let pow = min_set.pow();
        assert_eq!(pow, 12);

        let test_line_3 =
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = proc_line(test_line_3.to_string());
        let min_set: Round = game.get_min_cubes();
        let pow = min_set.pow();
        assert_eq!(pow, 1560);
    }
}
