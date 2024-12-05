use std::{
    collections::{BTreeMap, BTreeSet},
    result,
};

const INPUT: &'static str = include_str!("input.txt");

// struct Rule {
//     left: u32,
//     right: u32,
// }

fn main() {
    // let mut rules: Vec<Rule> = Vec::new();
    let mut rules: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    let mut correct_updates: Vec<Vec<u32>> = Vec::new();
    let mut rules_done = false;

    'outer: for line in INPUT.lines() {
        if line.len() == 0 {
            rules_done = true;
            continue;
        }

        if !rules_done {
            let mut parts = line.split('|');
            let left: u32 = parts.next().unwrap().parse().unwrap();
            let right: u32 = parts.next().unwrap().parse().unwrap();

            let after = rules.entry(left).or_default();
            after.insert(right);
            continue;
        }

        // Rules are parsed, check which updates are correct
        let pages: Vec<u32> = line.split(',').map(|x| x.parse().unwrap()).collect();
        let mut seen: BTreeSet<u32> = BTreeSet::new();
        for entry in pages.iter() {
            let comes_before = rules.get(entry);
            if let Some(comes_before) = comes_before {
                if !seen.is_disjoint(comes_before) {
                    // We've seen a page that should come after this one. Not correct.
                    continue 'outer;
                }
            }
            seen.insert(*entry);
        }
        correct_updates.push(pages.clone());
    }

    let middle_pages = correct_updates.iter().map(|x| {
        let middle = x.len() / 2;
        x[middle]
    });

    let result: u32 = middle_pages.sum();

    println!("Result: {}", result);
}
