use aoc::*;

#[parse(line)]
fn parse_directions(input: &str) -> Result<i32> {
    let (dir, amount) = input.split_at(1);
    let amount = amount.parse::<i32>()? * if dir == "L" { -1 } else { 1 };
    Ok(amount)
}

#[part_one]
fn count_zeros(steps: &[i32]) -> i32 {
    steps
        .iter()
        .fold((50, 0), |(dial, zeros), step| {
            let new_dial = (dial + step).rem_euclid(100);
            (new_dial, if new_dial == 0 { zeros + 1 } else { zeros })
        })
        .1
}

#[part_two]
fn count_moves_past_zero(steps: &[i32]) -> i32 {
    steps
        .iter()
        .fold((50, 0), |(dial, zeros), step| {
            let new_dial = dial + step;
            let mut new_zeros = zeros;
            if dial != 0 && new_dial != 0 && dial.signum() != new_dial.signum() {
                new_zeros += 1;
            }
            new_zeros += new_dial.abs() / 100;
            // edge case: if new_dial is a multiple of 100 and not 0, remove one zero
            if new_dial != 0 && new_dial % 100 == 0 {
                new_zeros -= 1;
            }
            new_zeros += if new_dial == 0 { 1 } else { 0 };
            let new_dial = (dial + step).rem_euclid(100);
            (new_dial, new_zeros)
        })
        .1
}

aoc_day!(1);
