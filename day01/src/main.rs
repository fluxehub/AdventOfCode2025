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
            let new_dial = (dial + step).rem_euclid(100);
            let to_reach_zero = if dial == 0 {
                100
            } else if *step > 0 {
                100 - dial
            } else {
                dial
            };
            let remainder_step = step.abs() - to_reach_zero;
            if remainder_step < 0 {
                (new_dial, zeros) // Didn't cross zero this time
            } else {
                (new_dial, zeros + 1 + remainder_step / 100)
            }
        })
        .1
}

aoc_day!(1);
