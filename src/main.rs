mod day8;

fn main() {
    let mut lines = Vec::new();
    for line in day8::read_file().unwrap() {
        let line = line.unwrap();
        lines.push(line);
    }

    let mut lines = day8::read_file().unwrap();

    println!(
        "star 1: {:#?}",
        day8::star1(lines.map(|s| s.unwrap()).collect())
    );
    // println!("star 2: {}", day8::star2(lines.clone()));
}
