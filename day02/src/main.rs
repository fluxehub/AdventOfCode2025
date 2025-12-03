use aoc::*;

#[parse]
fn get_ids(input: &str) -> Result<Vec<u64>> {
    input
        .split(",")
        .map(|s| {
            let (start, end) = s.split_once("-").ok_or_eyre("Missing separator")?;
            let start = start.trim().parse()?;
            let end = end.trim().parse()?;
            Ok(start..=end)
        })
        .flatten_ok()
        .collect()
}

#[part_one]
fn count_duplicated_patterns(ids: &[u64]) -> u64 {
    ids.iter()
        .filter(|id| {
            let id_string = id.to_string(); // Have to bind or we get temp value dropped :(
            let (first, second) = id_string.split_at(id_string.len() / 2);
            first == second
        })
        .sum()
}

#[part_two]
fn count_repeating_patterns(ids: &[u64]) -> u64 {
    ids.iter()
        .filter(|id| {
            let id_string = id.to_string();
            for pos in 1..=(id_string.len() / 2) {
                if id_string.as_bytes().chunks(pos).all_equal() {
                    return true;
                }
            }
            false
        })
        .sum()
}

aoc_day!(2);
