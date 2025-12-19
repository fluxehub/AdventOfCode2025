// Using Z3 is NOT CHEATING!!!!

use std::iter::Sum;

use aoc::*;
use rayon::prelude::*;
use regex::Regex;
use z3::{
    Optimize,
    ast::{BV, Bool, Int},
};

#[derive(Debug)]
struct Machine {
    lights_target: u64,
    joltage_target: Vec<u64>,
    buttons: Vec<u64>,
    size: u32,
}

#[parse(line)]
fn parse_machine(input: &str) -> Result<Machine> {
    let re = Regex::new(r"\[([.#]*)\] ((?:\(\d+(?:,\d+)*\) )+)\{(\d+(?:,\d+)*)\}")?;
    let (_, [target_string, buttons_string, joltage_string]) =
        re.captures(input).ok_or_eyre("Invalid input")?.extract();

    let size = target_string.len() as u32;
    let lights_target = target_string
        .chars()
        .enumerate()
        .fold(0u64, |acc, (i, c)| acc | ((c == '#') as u64) << i);

    let buttons = buttons_string
        .split_whitespace()
        .map(|button_str| {
            button_str[1..button_str.len() - 1]
                .split(',')
                .map(str::parse::<u32>)
                .try_fold(0u64, |acc, idx| idx.map(|i| acc | (1 << i)))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let joltage_target = joltage_string
        .split(",")
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Machine {
        lights_target,
        joltage_target,
        buttons,
        size,
    })
}

fn solve_machine_lights(machine: &Machine) -> usize {
    let optimizer = Optimize::new();

    let target = BV::from_u64(machine.lights_target, machine.size);
    let buttons: Vec<BV> = machine
        .buttons
        .iter()
        .map(|&b| BV::from_u64(b, machine.size))
        .collect();

    let mut expression = BV::from_u64(0, machine.size);
    let presses = (0..buttons.len())
        .map(|i| Bool::new_const(format!("press_{i}")))
        .collect_vec();
    for (button, press) in buttons.iter().zip(presses.iter()) {
        expression = Bool::ite(press, &expression.bvxor(button), &expression);
    }
    optimizer.assert(&expression.eq(&target));
    optimizer.minimize(&Int::sum(
        presses
            .iter()
            .map(|press| Bool::ite(press, &Int::from(1), &Int::from(0))),
    ));
    if optimizer.check(&[]) != z3::SatResult::Sat {
        panic!("Could not sat machine!");
    }
    let model = optimizer.get_model().unwrap();
    presses
        .iter()
        .filter(|&press| model.eval(press, false).unwrap().as_bool().unwrap())
        .count()
}

fn check_bit_set(bit: u64, at: usize) -> bool {
    bit & (1 << at) != 0
}

fn solve_machine_joltage(machine: &Machine) -> u64 {
    let optimizer = Optimize::new();

    let targets = machine
        .joltage_target
        .iter()
        .map(|&j| Int::from_u64(j))
        .collect_vec();
    let presses = (0..machine.buttons.len())
        .map(|i| Int::new_const(format!("press_{i}")))
        .collect_vec();

    // Each press amount must be non-negative
    for press in &presses {
        optimizer.assert(&press.ge(0));
    }

    for (joltage_idx, target) in targets.iter().enumerate() {
        let buttons_for_target =
            machine
                .buttons
                .iter()
                .enumerate()
                .filter_map(|(button_idx, &button)| {
                    if check_bit_set(button, joltage_idx) {
                        Some(&presses[button_idx])
                    } else {
                        None
                    }
                });
        optimizer.assert(&Int::sum(buttons_for_target).eq(target));
    }

    optimizer.minimize(&Int::sum(presses.iter()));
    if optimizer.check(&[]) != z3::SatResult::Sat {
        panic!("Could not sat machine!");
    }
    let model = optimizer.get_model().unwrap();
    presses
        .iter()
        .map(|p| model.eval(p, false).unwrap().as_u64().unwrap())
        .sum()
}

#[part_one]
fn find_fewest_presses_for_lights(machines: &[Machine]) -> usize {
    machines.par_iter().map(solve_machine_lights).sum()
}

#[part_two]
fn find_fewest_presses_for_joltage(machines: &[Machine]) -> u64 {
    machines.iter().map(solve_machine_joltage).sum()
}

aoc_day!(10);
