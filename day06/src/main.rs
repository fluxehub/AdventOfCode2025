use aoc::{color_eyre::eyre::bail, *};

#[part_one]
fn add_problems(input: &str) -> Result<u64> {
    let mut rows = vec![];
    let mut operators = vec![];

    for line in input.lines() {
        if !line
            .trim()
            .chars()
            .next()
            .ok_or_eyre("Empty line")?
            .is_numeric()
        {
            operators = line
                .split_whitespace()
                .map(|op| -> Result<fn(u64, u64) -> u64> {
                    match op {
                        "+" => Ok(|a, b| a + b),
                        "*" => Ok(|a, b| a * b),
                        a => bail!("Invalid operator {}", a),
                    }
                })
                .collect::<Result<Vec<_>>>()?;
        } else {
            rows.push(
                line.split_whitespace()
                    .map(|s| s.parse::<u64>())
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }
    }

    Ok(utils::transpose(rows)
        .into_iter()
        .zip(operators)
        .map(|(col, op)| col.iter().copied().reduce(op).unwrap())
        .sum())
}

#[part_two]
fn add_cephalopod_format(input: &str) -> Result<u64> {
    let mut lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // Extend all vecs to be the same length
    let max_len = lines.iter().map(|row| row.len()).max().unwrap();
    for row in &mut lines {
        row.extend(vec![' '; max_len - row.len()]);
    }
    let rotated: Vec<String> = utils::transpose(lines)
        .into_iter()
        .map(|chars| chars.into_iter().collect())
        .collect();

    rotated
        .split(|s| s.trim().is_empty())
        .map(|problem| {
            // Op is the last char of the first line
            let op = match problem[0].chars().last().ok_or_eyre("Empty row")? {
                '+' => |a: u64, b| a + b,
                '*' => |a: u64, b| a * b,
                c => bail!("Invalid operator: {}", c),
            };
            let first_row = problem[0][..problem[0].len() - 1].trim().parse::<u64>()?;
            problem[1..]
                .iter()
                .map(|s| s.trim().parse::<u64>())
                .try_fold(first_row, |acc, n| Ok(op(acc, n?)))
        })
        .sum()
}

aoc_day!(6);
