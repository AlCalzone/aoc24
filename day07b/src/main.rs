use std::time::Instant;

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
        let combinations = 3_u64.pow((numbers.len() - 1) as _);
        for combination in 0..combinations {
            let mut sum = 0;
            let mut combination = combination;
            for (i, num) in numbers.iter().enumerate() {
                if i == 0 {
                    sum = *num;
                    continue;
                }

                match combination % 3 {
                    0 => sum += num,
                    1 => sum *= num,
                    2 => {
                        let num_digits = (*num as f32).log10() as u32 + 1;
                        sum = 10_u64.pow(num_digits) * sum + num;
                    }
                    _ => unreachable!(),
                }
                combination /= 3;
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
