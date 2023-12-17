mod day17;

fn main() {
    let buf = day17::read_input("input/17.txt");
    let star1 = day17::star1(buf.clone());
    let star2 = day17::star2(buf.clone());
    println!("star 1: {:#?}", star1);
    println!("star 2: {:#?}", star2);
}
