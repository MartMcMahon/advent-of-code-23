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

#[derive(Debug)]
struct Box {
    lenses: Vec<Lens>,
}
impl Box {
    fn replace_or_add(&mut self, label: Vec<u8>, value: i32) {
        for (l_idx, lens) in self.lenses.iter().enumerate() {
            if lens.label == label {
                // replace lens
                self.lenses[l_idx] = Lens { label, value };
                return;
            }
        }

        self.lenses.push(Lens { label, value });
    }

    fn focus_power(&self, box_idx: i32) -> i32 {
        // One plus the box number of the lens in question.
        // The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
        // The focal length of the lens.

        let mut sum = 0;
        for (i, lens) in self.lenses.iter().enumerate() {
            let mut power = 1 + box_idx;
            power *= i as i32 + 1;
            power *= lens.value;
            sum += power;
        }
        sum
    }
}

#[derive(Debug)]
struct Lens {
    label: Vec<u8>,
    value: i32,
}
// impl Copy for Lens {}
// impl Clone for Lens {
//     fn clone(&self) -> Lens {
//         label: self.label.clone(),
//         value: self.value,
//     }
// }

pub fn star2(mut buf: Vec<u8>) -> i32 {
    let mut boxes = Vec::with_capacity(256);
    for i in 0..256 {
        boxes.push(Box { lenses: Vec::new() });
    }
    // Box {
    //     lenses: Vec::new().clone(),
    // }; 256];

    let mut label: Vec<u8> = Vec::new();
    let mut running_hash = 0;
    for mut i in 0..buf.len() {
        let c = buf[i];
        match c {
            b',' => { // end of step. reset things
            }
            b'\n' => continue,
            b'=' => {
                // replace or add
                boxes[running_hash].replace_or_add(
                    label.clone(),
                    i32::from_str_radix(&buf[i + 1..].to_owned(), 10).unwrap(),
                );
                i += 2;
            }
            b'-' => {
                // remove from box
                for mut i in 0..boxes[running_hash].lenses.len() {
                    let lens = &boxes[running_hash].lenses[i];
                    if lens.label == label {
                        // remove lens
                        boxes[running_hash].lenses.remove(i);
                        i -= 1;
                    }
                }
            }
            _ => {
                label.push(c);
                running_hash = hash_math(running_hash as i32, c) as usize;
            }
        }
    }

    let mut power = 0;
    for (i, b) in boxes.iter().enumerate() {
        power += b.focus_power(i as i32);
    }

    power
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

    #[test]
    fn test_star2() {
        let input = TEST_INPUT.as_bytes().to_vec();
        let x = star2(input);
        assert_eq!(x, 145);
    }
}
