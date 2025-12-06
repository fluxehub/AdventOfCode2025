use aoc::*;

// I really need a way to specify when no parse function is needed
#[parse]
fn parse_math(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[part_one]
fn add_problems(input: &[String]) -> u64 {
    let mut rows = vec![];
    let mut operators = vec![];

    for line in input {
        if !line.trim().chars().next().unwrap().is_numeric() {
            operators = line
                .split_whitespace()
                .map(|op| match op {
                    "+" => |a, b| a + b,
                    "*" => |a, b| a * b,
                    a => panic!("Invalid operator {}", a),
                })
                .collect();
        } else {
            rows.push(
                line.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect(),
            );
        }
    }

    utils::transpose(rows)
        .into_iter()
        .zip(operators)
        .map(|(col, op)| col.iter().copied().reduce(op).unwrap())
        .sum()
}

#[part_two]
fn add_cephalopod_format(input: &[String]) -> u64 {
    let mut lines: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
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
            let op = match problem[0].chars().last().unwrap() {
                '+' => |a, b| a + b,
                '*' => |a, b| a * b,
                _ => panic!("Invalid operator"),
            };
            let first_row = problem[0][..problem[0].len() - 1]
                .trim()
                .parse::<u64>()
                .unwrap();
            problem[1..]
                .iter()
                .map(|s| s.trim().parse::<u64>().unwrap())
                .fold(first_row, op)
        })
        .sum()
}

aoc_day!(6);
