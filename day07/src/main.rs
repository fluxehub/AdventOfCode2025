use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use aoc::*;
use petgraph::graph::{DiGraph, NodeIndex};

// Do not solve it this way

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Node {
    Start,
    Splitter,
    End,
}

type Manifold = DiGraph<Node, ()>;

#[parse(lines)]
fn parse_manifold(input: Lines) -> (NodeIndex, NodeIndex, Manifold) {
    let mut manifold = DiGraph::new();

    let mut beams: HashMap<usize, Vec<NodeIndex>> = HashMap::new();
    let start_node = manifold.add_node(Node::Start);

    for line in input {
        if all(line.chars(), |c| c == '.') {
            continue;
        }
        let mut new_beams = HashMap::new();
        let mut hit_splitters = HashSet::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    new_beams.insert(x, vec![start_node]);
                }
                '^' => {
                    let Some(prev_nodes) = beams.get(&x) else {
                        continue;
                    };
                    hit_splitters.insert(x);
                    let node = manifold.add_node(Node::Splitter);
                    for prev_node in prev_nodes {
                        manifold.add_edge(*prev_node, node, ());
                    }
                    if let Some(new_beams) = new_beams.get_mut(&(x - 1)) {
                        new_beams.push(node);
                    } else {
                        new_beams.insert(x - 1, vec![node]);
                    }
                    if let Some(new_beams) = new_beams.get_mut(&(x + 1)) {
                        new_beams.push(node);
                    } else {
                        new_beams.insert(x + 1, vec![node]);
                    }
                }
                _ => {}
            }
        }
        for (x, beams) in beams {
            if hit_splitters.contains(&x) {
                continue;
            }
            if let Some(new_beams) = new_beams.get_mut(&x) {
                new_beams.extend(beams);
            } else {
                new_beams.insert(x, beams);
            }
        }
        beams = new_beams;
    }
    // Add end node
    let end_node = manifold.add_node(Node::End);
    for (_, nodes) in beams {
        for node in nodes {
            manifold.add_edge(node, end_node, ());
        }
    }
    (start_node, end_node, manifold)
}

#[part_one]
fn count_beam_splits(_start: &NodeIndex, _end: &NodeIndex, manifold: &Manifold) -> usize {
    // Count the number of nodes with incoming edges
    manifold
        .node_indices()
        .filter(|&node| {
            manifold
                .neighbors_directed(node, petgraph::Direction::Incoming)
                .count()
                > 0
        })
        .count()
        - 1 // Minus one for the end node
}

#[part_two]
fn count_all_possible_paths(start: &NodeIndex, end: &NodeIndex, manifold: &Manifold) -> usize {
    let topological_order = petgraph::algo::toposort(manifold, None).unwrap();
    let mut dp = HashMap::new();
    for node in manifold.node_indices() {
        if node == *start {
            dp.insert(node, 1);
        } else {
            dp.insert(node, 0);
        }
    }

    for node in topological_order {
        for neighbor in manifold.neighbors_directed(node, petgraph::Direction::Outgoing) {
            *dp.get_mut(&neighbor).unwrap() += dp[&node];
        }
    }

    *dp.get(end).unwrap()
}

aoc_day!(7);
