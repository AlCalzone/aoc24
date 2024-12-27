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

    let len_3_cycles: BTreeSet<_> = nodes
        .iter()
        .enumerate()
        .flat_map(|(i, _)| find_cycles(&nodes, Default::default(), 3, i))
        .collect();

    // let cycle_nodes: Vec<_> = len_3_cycles
    //     .iter()
    //     .map(|c| c.iter().map(|i| nodes[*i].name).collect::<Vec<_>>())
    //     .collect();
    // println!("{:#?}", cycle_nodes);

    let result = len_3_cycles
        .iter()
        .filter(|c| c.iter().any(|i| nodes[*i].name.starts_with("t")))
        .count();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}

fn find_cycles(
    nodes: &[Node],
    mut visited: Vec<usize>,
    length: usize,
    current: usize,
) -> Vec<Vec<usize>> {
    visited.push(current);
    let node = &nodes[current];
    if visited.len() == length {
        if node.neighbors.contains(&visited[0]) {
            visited.sort_unstable();
            return vec![visited];
        } else {
            return vec![];
        }
    } else {
        node.neighbors
            .iter()
            .filter(|neighbor| !visited.contains(neighbor))
            .flat_map(|neighbor| find_cycles(nodes, visited.clone(), length, *neighbor))
            .collect()
    }
}
