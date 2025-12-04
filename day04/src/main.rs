use std::collections::HashSet;

use aoc::*;

#[derive(Debug, Clone)]
struct Floor {
    width: usize,
    height: usize,
    paper_tiles: HashSet<(usize, usize)>,
}

impl Floor {
    fn check_valid_coord(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn find_adjacent_paper(&self, x: usize, y: usize) -> usize {
        iproduct!(-1..=1, -1..=1)
            .filter(|(dx, dy)| *dx != 0 || *dy != 0)
            .filter(|(dx, dy)| {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                self.check_valid_coord(nx, ny) && self.paper_tiles.contains(&(nx, ny))
            })
            .count()
    }
}

#[parse(lines)]
fn parse_floor(input: Lines) -> Floor {
    let mut paper_tiles = HashSet::new();
    let lines = input.collect_vec();
    let size = (lines.len(), lines[0].len());

    // The functional version of this is not really any more readable (but when has that stopped me?)
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                paper_tiles.insert((x, y));
            }
        }
    }

    Floor {
        width: size.0,
        height: size.1,
        paper_tiles,
    }
}

#[part_one]
fn find_accessible_paper(floor: &Floor) -> usize {
    floor
        .paper_tiles
        .iter()
        .filter(|(x, y)| floor.find_adjacent_paper(*x, *y) < 4)
        .count()
}

#[part_two]
fn remove_paper(floor: &Floor) -> usize {
    // Funnier to do it as a big subtraction than to bind to vars
    floor.paper_tiles.len()
        - std::iter::successors(Some(floor.clone()), |floor| {
            let mut next_floor = floor.clone();
            next_floor.paper_tiles = floor
                .paper_tiles
                .iter()
                .filter(|(x, y)| floor.find_adjacent_paper(*x, *y) >= 4)
                .cloned()
                .collect();
            (next_floor.paper_tiles.len() != floor.paper_tiles.len()).then_some(next_floor)
        })
        .last()
        .unwrap()
        .paper_tiles
        .len()
}

aoc_day!(4);
