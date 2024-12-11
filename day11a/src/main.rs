use std::{collections::BTreeMap, time::Instant};

const INPUT: &'static str = include_str!("input.txt");
const ITERATIONS: usize = 25;

fn main() {
    let start = Instant::now();

    let numbers = INPUT
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut cache: BTreeMap<(u64, usize), u32> = BTreeMap::new();

    let result = numbers
        .iter()
        .map(|&num| simulate_num(num, 1, &mut cache))
        .sum::<u32>();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}

fn simulate_num(num: u64, iteration: usize, cache: &mut BTreeMap<(u64, usize), u32>) -> u32 {
    if let Some(&result) = cache.get(&(num, iteration)) {
        return result;
    }

    if num == 0 {
        if iteration == ITERATIONS {
            return 1;
        }
        return simulate_num(1, iteration + 1, cache);
    }

    let num_digits = num.ilog10() + 1;
    if num_digits % 2 == 0 {
        if iteration == ITERATIONS {
            return 2;
        }

        let mask = 10u64.pow(num_digits / 2);
        let first_half = num / mask;
        let second_half = num % mask;

        return simulate_num(first_half, iteration + 1, cache)
            + simulate_num(second_half, iteration + 1, cache);
    }

    if iteration == ITERATIONS {
        return 1;
    }
    return simulate_num(num * 2024, iteration + 1, cache);
}
