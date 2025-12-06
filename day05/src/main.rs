use aoc::*;
use rangemap::RangeInclusiveSet;

#[parse]
fn parse_ingredients(input: &str) -> Result<(RangeInclusiveSet<u64>, Vec<u64>)> {
    let (range_list, ingredient_list) = input.split_once("\n\n").ok_or_eyre("Invalid input")?;

    let ranges = range_list
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').ok_or_eyre("Invalid range")?;
            let start = start.parse()?;
            let end = end.parse()?;
            Ok(start..=end)
        })
        .collect::<Result<RangeInclusiveSet<_>>>()?;

    let ingredients = ingredient_list
        .lines()
        .map(&str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?;

    Ok((ranges, ingredients))
}

#[part_one]
fn count_fresh_ingredients(ranges: &RangeInclusiveSet<u64>, ingredients: &[u64]) -> usize {
    ingredients.iter().filter(|i| ranges.contains(i)).count()
}

#[part_two]
fn count_all_fresh_ingredients(ranges: &RangeInclusiveSet<u64>, _ingredients: &[u64]) -> usize {
    ranges.iter().map(|r| r.try_len().unwrap()).sum()
}

aoc_day!(5);
