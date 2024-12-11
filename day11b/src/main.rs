use rustc_hash::FxHashMap;
use std::{
    collections::{hash_map::Entry::*, BTreeSet},
    time::Instant,
};

const INPUT: &'static str = include_str!("input.txt");
const ITERATIONS: usize = 75;

type Num = u64;

#[derive(Clone, Copy)]
enum Next {
    Single(Num),
    Double(Num, Num),
}

fn main() {
    let start = Instant::now();

    let numbers = INPUT.split_whitespace().map(|x| x.parse::<Num>().unwrap());

    let mut lookup: FxHashMap<Num, Next> = FxHashMap::default();

    // Step 1: Build lookup table
    let mut todos: BTreeSet<_> = numbers.clone().collect();
    while let Some(todo) = todos.pop_first() {
        let mut current = todo;
        loop {
            match lookup.entry(current) {
                Vacant(vacant_entry) => {
                    let new_entry = next(current);
                    vacant_entry.insert(new_entry);
                    match new_entry {
                        Next::Single(num) => {
                            current = num;
                        }
                        Next::Double(first, second) => {
                            current = first;
                            todos.insert(second);
                        }
                    }
                }
                Occupied(_) => {
                    // We found a cycle
                    break;
                }
            }
        }
    }

    // Step 2: Count numbers for the first iteration
    let mut counts: FxHashMap<Num, usize> = FxHashMap::default();
    for num in numbers {
        *counts.entry(num).or_insert(0) += 1;
    }

    // Step 3: Simulate
    for _ in 0..ITERATIONS {
        let mut next_counts: FxHashMap<Num, usize> = FxHashMap::default();
        for (num, count) in counts.iter() {
            match lookup.get(&num).unwrap() {
                Next::Single(next) => {
                    *next_counts.entry(*next).or_insert(0) += count;
                }
                Next::Double(first, second) => {
                    *next_counts.entry(*first).or_insert(0) += count;
                    *next_counts.entry(*second).or_insert(0) += count;
                }
            }
        }
        counts = next_counts;
    }

    // Step 4: Count final numbers
    let result = counts.values().sum::<usize>();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
    println!("(cache size: {})", lookup.len());
}

fn next(num: Num) -> Next {
    if num == 0 {
        return Next::Single(1);
    }

    let num_digits = num.ilog10() + 1;
    if num_digits % 2 == 0 {
        let mask = 10u64.pow(num_digits / 2);
        let first_half = num / mask;
        let second_half = num % mask;

        return Next::Double(first_half, second_half);
    }

    return Next::Single(num * 2024);
}
