use std::{
    collections::{BTreeMap, BTreeSet},
    result,
};

const INPUT: &'static str = include_str!("input.txt");

// struct Rule {
//     left: u32,
//     right: u32,
// }

struct Entry {
    value: u32,
    comes_before: BTreeSet<u32>,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value == other.value {
            return std::cmp::Ordering::Equal;
        }
        if self.comes_before.contains(&other.value) {
            return std::cmp::Ordering::Less;
        }
        if other.comes_before.contains(&self.value) {
            return std::cmp::Ordering::Greater;
        }
        // No rules. Should not happen, but we'll just say they're equal.
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.value == other.value {
            return Some(std::cmp::Ordering::Equal);
        }
        if self.comes_before.contains(&other.value) {
            return Some(std::cmp::Ordering::Less);
        }
        if other.comes_before.contains(&self.value) {
            return Some(std::cmp::Ordering::Greater);
        }
        None
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Entry {}

fn main() {
    // let mut rules: Vec<Rule> = Vec::new();
    let mut rules: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    let mut incorrect_updates: Vec<Vec<u32>> = Vec::new();
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
                    incorrect_updates.push(pages.clone());
                    continue 'outer;
                }
            }
            seen.insert(*entry);
        }
    }

    // We've collected all incorrect updates. Sort them, then find their middle pages
    let sorted = incorrect_updates
        .iter()
        .map(|x| {
            let mut pages: Vec<_> = x
                .iter()
                .map(|v| {
                    let comes_before = match rules.get(v) {
                        Some(comes_before) => comes_before.clone(),
                        None => Default::default(),
                    };
                    Entry {
                        value: *v,
                        comes_before,
                    }
                })
                .collect();
            pages.sort();
            pages
        })
        .collect::<Vec<_>>();

    let middle_pages = sorted.iter().map(|x| {
        let middle = x.len() / 2;
        x[middle].value
    });

    let result: u32 = middle_pages.sum();

    println!("Result: {}", result);
}
