mod day15;

fn main() {
    let buf = day15::read_input("input/15.txt");
    let star1 = day15::star1(buf.clone());
    let star2 = day15::star2(buf.clone());
    println!("star 1: {:#?}", star1);
    println!("star 2: {:#?}", star2);
}
