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

pub mod utils {
    // Source - https://stackoverflow.com/a
    // Posted by Netwave, modified by community. See post 'Timeline' for change history
    // Retrieved 2025-12-06, License - CC BY-SA 4.0
    pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
        assert!(!v.is_empty());
        let len = v[0].len();
        let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().expect("All rows must be the same length"))
                    .collect::<Vec<T>>()
            })
            .collect()
    }
}

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

/// Default parse implementation for when no #[parse] is defined.
/// Local definitions from #[parse] will shadow these via wildcard import.
pub mod __aoc_defaults {
    use std::sync::OnceLock;

    pub static __PARSED_DATA: OnceLock<String> = OnceLock::new();

    // Dummy parse function for when no #[parse] is defined
    pub fn __parse_data(text: &str) {
        __PARSED_DATA.set(text.to_string()).unwrap();
    }
}

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
        // Wildcard import allows local #[parse] definitions to shadow these defaults
        #[allow(unused_imports)]
        use aoc::__aoc_defaults::*;

        fn main() -> Result<()> {
            color_eyre::install()?;
            __parse_data(&aoc::__get_input($day)?);
            aoc::__run_day();
            Ok(())
        }
    };
    ($day:expr, $input:expr) => {
        // Wildcard import allows local #[parse] definitions to shadow these defaults
        #[allow(unused_imports)]
        use aoc::__aoc_defaults::*;

        fn main() -> Result<()> {
            color_eyre::install()?;
            __parse_data(&$input);
            aoc::__run_day();
            Ok(())
        }
    };
}
