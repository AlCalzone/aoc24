use rayon::prelude::*;
use std::time::Instant;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let start = Instant::now();

    let secrets = INPUT
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let transformed = secrets
        .into_par_iter()
        .map(|s| {
            let mut s = s;
            for _ in 0..2000 {
                s = next(s);
            }
            s
        })
        .map(|s| s as usize);

    let result: usize = transformed.sum();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}

fn next(secret: u32) -> u32 {
    let mut secret = secret;
    secret ^= secret.wrapping_shl(6) & 0xFFFFFF;
    secret ^= secret >> 5;
    secret ^= secret.wrapping_shl(11) & 0xFFFFFF;
    secret
}
