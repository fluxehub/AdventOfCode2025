use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use aoc::*;
use kiddo::{KdTree, SquaredEuclidean};
use union_find::{QuickFindUf, UnionBySize, UnionFind};

type Point = (u32, u32, u32);
type BoxMap = KdTree<f64, 3>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HeapEntry {
    distance: ordered_float::OrderedFloat<f64>,
    point_a: usize,
    point_b: usize,
}

#[parse(lines)]
fn parse_junction_boxes(input: Lines) -> Result<(Vec<Point>, BoxMap)> {
    let mut points = vec![];
    let mut tree = KdTree::new();
    for (idx, line) in input.enumerate() {
        let [x, y, z] = line
            .trim()
            .split(',')
            .map(|coord| coord.parse::<f64>().unwrap())
            .collect_array()
            .unwrap();
        points.push((x as u32, y as u32, z as u32));
        tree.add(&[x, y, z], idx as u64);
    }
    Ok((points, tree))
}

fn get_sorted_pairs(points: &[Point], box_map: &BoxMap) -> BinaryHeap<Reverse<HeapEntry>> {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    for (index, point) in points.iter().enumerate() {
        const NEIGHBOR_COUNT: usize = 6;
        let closest_neighbors = box_map.nearest_n::<SquaredEuclidean>(
            &[point.0 as f64, point.1 as f64, point.2 as f64],
            NEIGHBOR_COUNT,
        );
        // Skip the first neighbor
        for neighbor in closest_neighbors.iter().skip(1) {
            if seen.contains(&(index, neighbor.item as usize)) {
                continue;
            }
            heap.push(Reverse(HeapEntry {
                distance: neighbor.distance.into(),
                point_a: index,
                point_b: neighbor.item as usize,
            }));
            seen.insert((neighbor.item as usize, index));
        }
    }

    heap
}

#[part_one]
fn connect_closest_boxes(points: &[Point], box_map: &BoxMap) -> u64 {
    let mut union_find = QuickFindUf::<UnionBySize>::new(points.len());

    const MAX_ITERATIONS: usize = 999;
    let mut iterations = 0;

    let mut heap = get_sorted_pairs(points, box_map);

    while let Some(Reverse(entry)) = heap.pop() {
        iterations += 1;

        union_find.union(entry.point_a, entry.point_b);
        if iterations >= MAX_ITERATIONS {
            break;
        }
    }

    let mut circuit_sizes = vec![0; union_find.size()];

    for id in 0..points.len() {
        circuit_sizes[union_find.find(id)] += 1;
    }

    circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));

    circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2]
}

#[part_two]
fn connect_all(points: &[Point], box_map: &BoxMap) -> u32 {
    let mut union_find = QuickFindUf::<UnionBySize>::new(points.len());

    let mut heap = get_sorted_pairs(points, box_map);
    let mut last_points = 0;
    while let Some(Reverse(entry)) = heap.pop() {
        if !union_find.union(entry.point_a, entry.point_b) {
            continue;
        }

        last_points = points[entry.point_a].0 * points[entry.point_b].0;
    }
    last_points
}

aoc_day!(8);
