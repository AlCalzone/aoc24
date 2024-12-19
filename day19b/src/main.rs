use rustc_hash::{FxHashMap, FxHashSet};
use std::time::Instant;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let start = Instant::now();

    let (patterns, words) = INPUT.split_once("\n\n").unwrap();
    let patterns: FxHashSet<_> = patterns.split(", ").collect();
    let words: Vec<_> = words.lines().collect();

    let mut cache: FxHashMap<&str, usize> = FxHashMap::default();

    let counts: Vec<_> = words
        .iter()
        .map(|word| (word, check_word(word, &mut cache, &patterns)))
        .collect();

    // for (word, count) in &counts {
    //     println!("{word}: {count}");
    // }

    let result: usize = counts.iter().map(|(_, count)| *count).sum();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}

fn check_word<'a>(
    word: &'a str,
    cache: &mut FxHashMap<&'a str, usize>,
    patterns: &FxHashSet<&'a str>,
) -> usize {
    if let Some(ret) = cache.get(word) {
        return *ret;
    }

    let mut ret: usize = 0;

    if patterns.contains(word) {
        ret += 1;
    }

    for pat in patterns {
        if let Some(suffix) = word.strip_prefix(pat) {
            ret += check_word(suffix, cache, patterns);
        }
    }

    cache.insert(word, ret);
    ret
}
