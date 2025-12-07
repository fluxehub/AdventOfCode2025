use std::{collections::HashMap, hash::Hash};

use aoc::*;
use petgraph::{
    Direction,
    graph::{DiGraph, NodeIndex},
};

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
    let mut beams = HashMap::new();
    let mut new_beams: HashMap<_, Vec<NodeIndex>> = HashMap::new();
    let start_node = manifold.add_node(Node::Start);

    for line in input {
        if line.chars().all(|c| c == '.') {
            continue;
        }
        new_beams.clear();

        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    new_beams.entry(x).or_default().push(start_node);
                }
                '^' => {
                    if let Some(prev_nodes) = beams.remove(&x) {
                        let node = manifold.add_node(Node::Splitter);
                        for &prev_node in &prev_nodes {
                            manifold.add_edge(prev_node, node, ());
                        }
                        new_beams.entry(x - 1).or_default().push(node);
                        new_beams.entry(x + 1).or_default().push(node);
                    }
                }
                _ => {}
            }
        }

        for (x, nodes) in beams.drain() {
            new_beams.entry(x).or_default().extend(nodes);
        }

        std::mem::swap(&mut beams, &mut new_beams);
    }

    let end_node = manifold.add_node(Node::End);
    for nodes in beams.into_values() {
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
                .neighbors_directed(node, Direction::Incoming)
                .count()
                > 0
        })
        .count()
        - 1 // Minus one for the end node
}

#[part_two]
fn count_all_possible_paths(start: &NodeIndex, end: &NodeIndex, manifold: &Manifold) -> usize {
    let topological_order = petgraph::algo::toposort(manifold, None).unwrap();
    let mut dp: HashMap<NodeIndex, usize> = manifold
        .node_indices()
        .map(|node| (node, if node == *start { 1 } else { 0 }))
        .collect();

    for node in topological_order {
        for neighbor in manifold.neighbors_directed(node, petgraph::Direction::Outgoing) {
            *dp.get_mut(&neighbor).unwrap() += dp[&node];
        }
    }

    dp[end]
}

aoc_day!(7);
