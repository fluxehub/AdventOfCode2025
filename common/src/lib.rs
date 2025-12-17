// Re-export some common libs and imports
pub use aoc_macro::{parse, part_one, part_two};
pub use color_eyre;
pub use color_eyre::{Result, eyre::OptionExt, eyre::bail};
pub use criterion;
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
    // Use workspace root for cache (where Cargo.toml with [workspace] lives)
    let cache_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("common crate should be in workspace")
        .join(".input");

    // Check for --example flag
    let use_example = std::env::args().any(|arg| arg == "--example");

    if use_example {
        let example_path = cache_dir.join(format!("day{day}_example"));
        return std::fs::read_to_string(&example_path)
            .map_err(|e| color_eyre::eyre::eyre!("Failed to read example input: {}", e));
    }

    // Try and load from .input/{day}
    let cache_path = cache_dir.join(format!("day{day}"));
    if let Ok(input) = std::fs::read_to_string(&cache_path) {
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

    std::fs::create_dir_all(&cache_dir)?;
    let mut file = File::create(&cache_path)?;
    file.write_all(input.as_bytes())?;

    Ok(input)
}

pub struct AocPart {
    pub part: u8,
    pub func: fn() -> String,
}

inventory::collect!(AocPart);

pub struct AocBench {
    pub part: u8,
    pub func: fn(&str) -> String,
}

inventory::collect!(AocBench);

/// Default parse implementation for when no #[parse] is defined.
/// Local definitions from #[parse] will shadow these via wildcard import.
pub mod __aoc_defaults {
    use std::sync::OnceLock;

    pub static __PARSED_DATA: OnceLock<String> = OnceLock::new();

    /// Returns parsed data (can be called multiple times, for benchmarks)
    pub fn __do_parse(text: &str) -> String {
        text.to_string()
    }

    pub fn __parse_data(text: &str) {
        __PARSED_DATA.set(__do_parse(text)).unwrap();
    }
}

pub fn __run_day() {
    std::thread::scope(|s| {
        let handles: Vec<_> = inventory::iter::<AocPart>
            .into_iter()
            .sorted_by_key(|p| p.part)
            .map(|part| (part.part, s.spawn(|| (part.func)())))
            .collect();

        for (part, handle) in handles {
            println!("Part {}: {}", part, handle.join().unwrap());
        }
    });
}

pub fn __run_benchmarks(day: u32, input: &str) {
    use criterion::Criterion;
    use std::time::Duration;

    let mut criterion = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .configure_from_args();

    for bench in inventory::iter::<AocBench>
        .into_iter()
        .sorted_by_key(|b| b.part)
    {
        let name = format!("day{:02} part {}", day, bench.part);
        criterion.bench_function(&name, |b| {
            b.iter(|| (bench.func)(std::hint::black_box(input)))
        });
    }

    criterion.final_summary();
}

#[macro_export]
macro_rules! aoc_day {
    ($day:expr) => {
        #[allow(unused_imports)]
        use aoc::__aoc_defaults::*;

        fn main() -> Result<()> {
            let input = aoc::__get_input($day)?;

            if std::env::args().any(|arg| arg == "--bench") {
                aoc::__run_benchmarks($day, &input);
                return Ok(());
            }

            color_eyre::install()?;
            __parse_data(&input);
            aoc::__run_day();
            Ok(())
        }
    };
    ($day:expr, $input:expr) => {
        #[allow(unused_imports)]
        use aoc::__aoc_defaults::*;

        fn main() -> Result<()> {
            let input: &str = &$input;

            if std::env::args().any(|arg| arg == "--bench") {
                aoc::__run_benchmarks($day, input);
                return Ok(());
            }

            color_eyre::install()?;
            __parse_data(input);
            aoc::__run_day();
            Ok(())
        }
    };
}
