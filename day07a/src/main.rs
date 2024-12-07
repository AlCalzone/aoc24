use std::time::Instant;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let start = Instant::now();

    let result: u64 = INPUT
        .lines()
        .filter_map(|line| {
            let (test_val, numbers) = line.split_once(": ").unwrap();
            let test_val = test_val.parse::<u64>().unwrap();
            let numbers = numbers
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            test(test_val, &numbers).then_some(test_val)
        })
        .sum();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}

fn test(test_val: u64, numbers: &[u64]) -> bool {
    match numbers {
        [last] => *last == test_val,
        [rest @ .., last] => {
            // Test multiplication
            if test_val % last == 0 && test(test_val / last, rest) {
                return true;
            }
            // Test addition
            if test_val >= *last && test(test_val - last, rest) {
                return true;
            }

            false
        }
        _ => unreachable!(),
    }
}
