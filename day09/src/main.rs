use aoc::*;
use geo::{Coord, Covers, LineString, Polygon, Rect, coord};

// I was gonna hand roll my own geo (there's some really cool optimizations you can do since it's all 90 degree angles)
// but the edge cases we're too annoying to deal with

#[parse(line)]
fn parse_points(input: &str) -> Result<Coord> {
    let (x, y) = input.split_once(',').ok_or_eyre("Invalid input")?;
    Ok(coord!(x: x.parse()?, y: y.parse()?))
}

fn find_all_boxes(points: &[Coord]) -> impl Iterator<Item = Rect> {
    points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| Rect::new(*a, *b))
}

fn rect_area(rect: &Rect) -> u64 {
    (rect.width() as u64 + 1) * (rect.height() as u64 + 1)
}

#[part_one]
fn find_largest_area(points: &[Coord]) -> u64 {
    find_all_boxes(points).map(|r| rect_area(&r)).max().unwrap()
}

#[part_two]
fn find_largest_area_in_polygon(points: &[Coord]) -> u64 {
    let polygon = Polygon::new(LineString::from(points.to_vec()), vec![]);
    rect_area(
        &find_all_boxes(points)
            .sorted_by_key(rect_area)
            .rev()
            .find(|r| polygon.covers(r))
            .unwrap(),
    )
}

aoc_day!(9);
