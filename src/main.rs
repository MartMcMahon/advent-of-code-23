mod day16;

fn main() {
    let buf = day16::read_input("input/16.txt");
    let star1 = day16::star1(buf.clone());
    let star2 = day16::star2(buf.clone());
    println!("star 1: {:#?}", star1);
    println!("star 2: {:#?}", star2);
}
