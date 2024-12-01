use std::collections::BTreeMap;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let (mut left, right): (Vec<u32>, Vec<u32>) = INPUT
        .lines()
        .map(|l| {
            let mut split = l.split_ascii_whitespace();
            let (left, right) = (split.next().unwrap(), split.next().unwrap());
            (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
        })
        .collect();
    left.sort();

    let mut count = BTreeMap::<u32, u32>::new();
    right
        .iter()
        .for_each(|&x| *count.entry(x).or_insert(0) += 1);

    let score: u32 = left.iter().map(|l| l * count.get(l).unwrap_or(&0)).sum();

    println!("Score: {}", score);
}
