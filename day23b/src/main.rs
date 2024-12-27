use std::{
    collections::{BTreeMap, BTreeSet},
    time::Instant,
};

const INPUT: &'static str = include_str!("input.txt");

struct Node {
    name: &'static str,
    neighbors: Vec<usize>,
}

fn main() {
    let start = Instant::now();

    let edges: Vec<_> = INPUT
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .collect();

    let node_set: BTreeSet<_> = edges.iter().flat_map(|(a, b)| [*a, *b]).collect();
    let mut nodes: Vec<_> = node_set
        .iter()
        .map(|node| Node {
            name: node,
            neighbors: vec![],
        })
        .collect();

    let indizes: BTreeMap<&str, usize> = node_set
        .iter()
        .enumerate()
        .map(|(i, node)| (*node, i))
        .collect();

    let edge_indizes: BTreeSet<_> = edges
        .iter()
        .map(|(a, b)| (*indizes.get(a).unwrap(), *indizes.get(b).unwrap()))
        .collect();

    for (a, b) in edge_indizes {
        nodes[a].neighbors.push(b);
        nodes[b].neighbors.push(a);
    }

    let mut cliques = find_cliques(&nodes);
    // find longest clique
    cliques.sort_by_key(|c| c.len());
    let mut longest_clique: Vec<_> = cliques
        .last()
        .unwrap()
        .iter()
        .map(|i| nodes[*i].name)
        .collect();
    longest_clique.sort();

    let result = longest_clique.join(",");

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}

fn find_cliques(nodes: &[Node]) -> Vec<Vec<usize>> {
    let mut cliques = Vec::new();
    let mut current_clique = Vec::new();
    let mut candidates: Vec<usize> = (0..nodes.len()).collect();
    let mut already_found = Vec::new();

    bron_kerbosch(
        nodes,
        &mut cliques,
        &mut current_clique,
        &mut candidates,
        &mut already_found,
    );
    cliques
}

fn bron_kerbosch(
    nodes: &[Node],
    cliques: &mut Vec<Vec<usize>>,
    current_clique: &mut Vec<usize>,
    candidates: &mut Vec<usize>,
    already_found: &mut Vec<usize>,
) {
    if candidates.is_empty() && already_found.is_empty() {
        cliques.push(current_clique.clone());
        return;
    }

    let candidates_clone = candidates.clone();
    for &v in &candidates_clone {
        current_clique.push(v);
        let mut new_candidates = Vec::new();
        let mut new_already_found = Vec::new();

        for &u in candidates.iter() {
            if nodes[v].neighbors.contains(&u) {
                new_candidates.push(u);
            }
        }

        for &u in already_found.iter() {
            if nodes[v].neighbors.contains(&u) {
                new_already_found.push(u);
            }
        }

        bron_kerbosch(
            nodes,
            cliques,
            current_clique,
            &mut new_candidates,
            &mut new_already_found,
        );

        current_clique.pop();
        candidates.retain(|&x| x != v);
        already_found.push(v);
    }
}
