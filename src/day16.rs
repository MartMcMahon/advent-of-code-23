use std::fs::File;
use std::io::prelude::*;

pub fn read_input(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    buf
}

pub fn star1(buf: Vec<u8>) -> i32 {
    0
}

pub fn star2(buf: Vec<u8>) -> i32 {
    0
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

    #[test]
    fn test_star1() {
        let input = TEST_INPUT.as_bytes().to_vec();
        let x = star1(input);
        assert_eq!(x, 46);
    }

    #[test]
    fn test_star2() {
        let input = TEST_INPUT.as_bytes().to_vec();
        let x = star2(input);
        // assert_eq!(x, 10);
    }
}
