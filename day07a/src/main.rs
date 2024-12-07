use std::{result, time::Instant};

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let start = Instant::now();

    let mut result: u64 = 0;

    for line in INPUT.lines() {
        let (test_val, numbers) = line.split_once(": ").unwrap();
        let test_val = test_val.parse::<u64>().unwrap();
        let numbers = numbers
            .split_whitespace()
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let combinations = 2_u64.pow((numbers.len() - 1) as _);
        for combination in 0..combinations {
            let mut sum = 0;
            for (i, num) in numbers.iter().enumerate() {
                if i == 0 {
                    sum = *num;
                } else if (combination & (1 << (i - 1))) != 0 {
                    sum += num;
                } else {
                    sum *= num;
                }
            }
            if sum == test_val {
                result += test_val;
                break;
            }
        }
    }

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}
