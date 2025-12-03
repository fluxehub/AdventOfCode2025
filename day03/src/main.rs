use aoc::*;

#[parse(line)]
fn parse_bank(line: &str) -> Result<Vec<u32>> {
    line.trim()
        .chars()
        .map(|c| c.to_digit(10).ok_or_eyre("Invalid digit"))
        .collect()
}

fn find_max_joltage(bank: &[u32], digits: usize) -> u64 {
    (0..digits)
        .rev()
        .fold((0, 0), |(joltage, start_pos), digit| {
            let max_set = bank[start_pos..bank.len() - digit]
                .iter()
                .enumerate()
                .rev() // Need first max
                .max_by_key(|(_, val)| *val);
            let (max_pos, max) = max_set.unwrap(); // Have to bind here to avoid temporary value being dropped :(
            (
                joltage + (*max) as u64 * 10_u64.pow(digit as u32),
                start_pos + max_pos + 1,
            )
        })
        .0
}

#[part_one]
fn find_two_digit_joltage(banks: &[Vec<u32>]) -> u64 {
    banks.iter().map(|bank| find_max_joltage(bank, 2)).sum()
}

#[part_two]
fn find_twelve_digit_joltage(banks: &[Vec<u32>]) -> u64 {
    banks.iter().map(|bank| find_max_joltage(bank, 12)).sum()
}

aoc_day!(3);
