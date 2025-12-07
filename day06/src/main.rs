use aoc::*;

fn parse_op(c: char) -> Result<fn(u64, u64) -> u64> {
    match c {
        '+' => Ok(|a, b| a + b),
        '*' => Ok(|a, b| a * b),
        c => bail!("Invalid operator: {}", c),
    }
}

#[part_one]
fn add_problems(input: &str) -> Result<u64> {
    let mut rows = vec![];
    let mut operators = vec![];

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(|c: char| c.is_ascii_digit()) {
            rows.push(
                trimmed
                    .split_whitespace()
                    .map(|s| s.parse())
                    .collect::<Result<_, _>>()?,
            );
        } else {
            operators = trimmed
                .split_whitespace()
                .map(|op| parse_op(op.chars().next().unwrap()))
                .collect::<Result<_>>()?;
        }
    }

    Ok(utils::transpose(rows)
        .into_iter()
        .zip(operators)
        .map(|(col, op)| col.into_iter().reduce(op).unwrap())
        .sum())
}

#[part_two]
fn add_cephalopod_format(input: &str) -> Result<u64> {
    let mut lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let max_len = lines.iter().map(|row| row.len()).max().unwrap_or(0);
    for row in &mut lines {
        row.resize(max_len, ' ');
    }

    let rotated: Vec<String> = utils::transpose(lines)
        .into_iter()
        .map(|chars| chars.into_iter().collect())
        .collect();

    rotated
        .split(|s| s.trim().is_empty())
        .map(|problem| {
            let first_line = &problem[0];
            let op = parse_op(first_line.chars().last().ok_or_eyre("Empty row")?)?;
            let first_val = first_line[..first_line.len() - 1].trim().parse::<u64>()?;

            problem[1..]
                .iter()
                .map(|s| s.trim().parse::<u64>())
                .try_fold(first_val, |acc, n| Ok(op(acc, n?)))
        })
        .sum()
}

aoc_day!(6);
