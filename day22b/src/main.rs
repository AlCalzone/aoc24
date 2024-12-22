#![feature(array_windows)]

use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::time::Instant;

const INPUT: &'static str = include_str!("input.txt");
const NUM_PRICES: usize = 2000;
const SEQ_LEN: usize = 4;

type PriceMap = FxHashMap<[i32; SEQ_LEN], i32>;

fn main() {
    let start = Instant::now();

    let secrets = INPUT
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // List of prices and price changes for each vendor
    let prices: Vec<Vec<(i32, i32)>> = secrets
        .into_par_iter()
        .map(|s| {
            let mut s = s;
            let mut price = s % 10;
            let mut ret = Vec::with_capacity(NUM_PRICES - 1);
            for _ in 0..NUM_PRICES - 1 {
                s = next(s);
                let next_price = s % 10;
                ret.push((next_price, next_price - price));
                price = next_price;
            }
            ret
        })
        .collect();

    // How many bananas each vendor would pay after each sequence
    let price_by_sequence: Vec<_> = prices
        .into_par_iter()
        .map(|prices| {
            let mut ret: PriceMap = PriceMap::default();
            for seq in prices.array_windows::<SEQ_LEN>() {
                let price = seq.last().unwrap().0;
                let changes: [i32; SEQ_LEN] = seq.map(|(_, change)| change);
                ret.entry(changes.clone()).or_insert(price);
            }
            ret
        })
        .collect();

    // How many bananas total each sequence would net
    let mut total_prices: PriceMap = PriceMap::default();
    for seller_prices in price_by_sequence.iter() {
        for (seq, price) in seller_prices.iter() {
            total_prices
                .entry(*seq)
                .and_modify(|p| *p += price)
                .or_insert(*price);
        }
    }

    // The best sequence
    let best = total_prices
        .iter()
        .max_by(|(_, &p1), (_, &p2)| p1.cmp(&p2))
        .unwrap();

    let result = best.1;

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("best sequence: {:?}", best.0);
    println!("(took: {:?})", elapsed);
}

fn next(secret: i32) -> i32 {
    let mut secret = secret;
    secret ^= secret.wrapping_shl(6) & 0xFFFFFF;
    secret ^= secret >> 5;
    secret ^= secret.wrapping_shl(11) & 0xFFFFFF;
    secret
}
