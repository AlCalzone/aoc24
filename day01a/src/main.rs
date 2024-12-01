const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = INPUT
        .lines()
        .map(|l| {
            let mut split = l.split_ascii_whitespace();
            let (left, right) = (split.next().unwrap(), split.next().unwrap());
            (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
        })
        .collect();
    left.sort();
    right.sort();

    let distance: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| r.abs_diff(*l))
        .sum();

    println!("Distance: {}", distance);
}
