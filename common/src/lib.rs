// Re-export some common libs and imports
pub use aoc_macro::{parse, part_one, part_two};
pub use color_eyre;
pub use color_eyre::{Result, eyre::OptionExt};
pub use inventory;
pub use itertools::Itertools;
pub use itertools::*;
pub use std::str::Lines;

use reqwest::blocking::Client;
use std::{fs::File, io::Write};

pub fn __get_input(day: u32) -> Result<String> {
    // Try and load from .input/{day}
    if let Ok(input) = std::fs::read_to_string(format!(".input/day{day}")) {
        return Ok(input);
    }

    // Otherwise pull from AOC server
    let client = Client::new();
    let session_token = include_str!("../../.session").trim();
    let input = client
        .get(format!("https://adventofcode.com/2025/day/{day}/input"))
        .header("Cookie", format!("session={session_token}"))
        .send()?
        .text()?;

    std::fs::create_dir_all(".input")?;
    let mut file = File::create(format!(".input/day{day}"))?;
    file.write_all(input.as_bytes())?;

    Ok(input)
}

pub struct AocPart {
    pub part: u8,
    pub func: fn(),
}

inventory::collect!(AocPart);

pub fn __run_day() {
    let parts = inventory::iter::<AocPart>
        .into_iter()
        .sorted_by_key(|p| p.part);
    for part in parts {
        (part.func)();
    }
}

#[macro_export]
macro_rules! aoc_day {
    ($day:expr) => {
        fn main() -> Result<()> {
            color_eyre::install()?;
            __parse_data(&aoc::__get_input($day)?);
            aoc::__run_day();
            Ok(())
        }
    };
    ($day:expr, $input:expr) => {
        fn main() -> Result<()> {
            color_eyre::install()?;
            __parse_data(&$input);
            aoc::__run_day();
            Ok(())
        }
    };
}
