use rustc_hash::{FxHashMap, FxHashSet};
use std::time::Instant;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let start = Instant::now();

    let (patterns, words) = INPUT.split_once("\n\n").unwrap();
    let patterns: FxHashSet<_> = patterns.split(", ").collect();
    let words: Vec<_> = words.lines().collect();

    let mut cache: FxHashMap<&str, bool> = FxHashMap::default();

    let possible: Vec<_> = words
        .iter()
        .filter(|word| check_word(word, &mut cache, &patterns))
        .collect();

    let result = possible.len();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}

fn check_word<'a>(
    word: &'a str,
    cache: &mut FxHashMap<&'a str, bool>,
    patterns: &FxHashSet<&'a str>,
) -> bool {
    if let Some(ret) = cache.get(word) {
        return *ret;
    }

    if patterns.contains(word) {
        cache.insert(word, true);
        return true;
    }

    for pat in patterns {
        if let Some(suffix) = word.strip_prefix(pat) {
            if check_word(suffix, cache, patterns) {
                return true;
            }
        }
    }

    cache.insert(word, false);
    false
}
