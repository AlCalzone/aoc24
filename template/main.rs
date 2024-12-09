use std::time::Instant;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let start = Instant::now();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}
