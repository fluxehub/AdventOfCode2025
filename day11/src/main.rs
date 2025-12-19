use aoc::*;
use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

type DeviceMap = DiGraph<String, ()>;

#[parse(lines)]
fn parse_reactor_devices(input: Lines) -> (DeviceMap, HashMap<String, NodeIndex>) {
    let mut graph = DiGraph::new();
    let mut node_indices = HashMap::new();

    for line in input {
        let parts = line.split_whitespace().collect_vec();
        let source_name = parts[0][0..parts[0].len() - 1].to_string();
        let output_names = parts[1..]
            .iter()
            .map(|&output| output.to_string())
            .collect_vec();

        let source_node_index = *node_indices
            .entry(source_name.clone())
            .or_insert_with(|| graph.add_node(source_name));

        for output_name in output_names {
            let output_node_index = *node_indices
                .entry(output_name.clone())
                .or_insert_with(|| graph.add_node(output_name));

            graph.add_edge(source_node_index, output_node_index, ());
        }
    }
    (graph, node_indices)
}

#[part_one]
fn find_all_paths(devices: &DeviceMap, node_indices: &HashMap<String, NodeIndex>) -> u32 {
    // Literally repeating day 7
    let mut dp = vec![0; devices.node_count()];
    let start = node_indices["you"];
    let end = node_indices["out"];

    dp[start.index()] = 1;

    // We do need to topo sort this time though
    let topo = toposort(devices, None).unwrap();

    for node in topo {
        let num_paths = dp[node.index()];
        for neighbor_index in devices.neighbors_directed(node, petgraph::Direction::Outgoing) {
            dp[neighbor_index.index()] += num_paths;
        }
    }

    dp[end.index()]
}

#[part_two]
fn find_all_paths_via_dsp(devices: &DeviceMap, node_indices: &HashMap<String, NodeIndex>) -> usize {
    let mut dp = vec![vec![0; 4]; devices.node_count()];
    let start = node_indices["svr"];
    let end = node_indices["out"];
    let fft = node_indices["fft"];
    let dac = node_indices["dac"];

    dp[start.index()][0] = 1;

    let topo = toposort(devices, None).unwrap();

    for node in topo {
        for (mask, num_paths) in dp[node.index()].clone().into_iter().enumerate() {
            for neighbor_index in devices.neighbors_directed(node, petgraph::Direction::Outgoing) {
                let mask = if neighbor_index == fft {
                    mask | 1
                } else if neighbor_index == dac {
                    mask | 2
                } else {
                    mask
                };
                dp[neighbor_index.index()][mask] += num_paths;
            }
        }
    }

    dp[end.index()][3]
}

aoc_day!(11);
