use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn star1(mut buf: Vec<u8>) -> i32 {
    let mut sum = 0;
    let mut running_total = 0;
    for c in &mut buf {
        if *c == b',' {
            sum += running_total;
            running_total = 0;
        } else if *c == b'\n' {
            continue;
        } else {
            running_total = hash_math(running_total, *c);
        }
    }
    // add last bit. no comma at end
    sum + running_total
}

pub fn star2(mut buf: Vec<u8>) -> i32 {
    0
}

pub fn hash_math(start: i32, char: u8) -> i32 {
    let mut ans = start;
    ans += char as i32;
    ans *= 17;
    ans %= 256;
    ans
}

pub fn read_input(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf);
    buf
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash_math() {
        let input = "HASH".as_bytes().to_vec();
        let mut x = 0;
        for c in input {
            x = hash_math(x, c);
        }
        assert_eq!(x, 52);
    }

    #[test]
    fn test_star1() {
        let input = TEST_INPUT.as_bytes().to_vec();
        let x = star1(input);
        assert_eq!(x, 1320);
    }
}
