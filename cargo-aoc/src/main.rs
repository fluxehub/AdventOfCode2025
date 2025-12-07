use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser, Subcommand};
use std::io::{Read, Write};
use std::process::Command;

const STYLES: Styles = Styles::styled()
    .header(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::BrightGreen.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default())
    .error(AnsiColor::BrightRed.on_default().effects(Effects::BOLD))
    .invalid(AnsiColor::BrightYellow.on_default().effects(Effects::BOLD))
    .valid(AnsiColor::BrightCyan.on_default().effects(Effects::BOLD));

#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum Cargo {
    Aoc(AocArgs),
}

#[derive(Parser)]
#[command(
    version,
    about = "Advent of Code runner and benchmarker",
    override_usage = "\x1b[1;96mcargo aoc \x1b[0;36m<DAY> [--example]\x1b[0m\n       \x1b[1;96mcargo aoc \x1b[0;36m<COMMAND>\x1b[0m",
    styles = STYLES
)]
struct AocArgs {
    #[command(subcommand)]
    command: AocCommand,
}

#[derive(Subcommand)]
enum AocCommand {
    /// Run a day's solution
    Run {
        #[arg(value_name = "DAY")]
        day: String,
        /// Use example input
        #[arg(long)]
        example: bool,
    },
    /// Run benchmarks for a day
    Bench {
        #[arg(value_name = "DAY")]
        day: String,
    },
    /// Create a new day project
    New {
        #[arg(value_name = "DAY")]
        day: String,
    },
    #[command(external_subcommand)]
    External(Vec<String>),
}

fn parse_day(day: &str) -> Option<(String, u32)> {
    let num: u32 = day.strip_prefix("day").unwrap_or(day).parse().ok()?;
    Some((format!("day{num:02}"), num))
}

fn ensure_example_input(day_num: u32) {
    let example_path = format!(".input/day{}_example", day_num);

    if std::fs::metadata(&example_path).is_ok() {
        return;
    }

    eprintln!("No example input found for day {}.", day_num);
    eprintln!("Paste example input, then press Enter followed by Ctrl+D:");
    eprintln!("---");

    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read input");
    std::fs::create_dir_all(".input").expect("Failed to create .input directory");
    let mut file = std::fs::File::create(&example_path).expect("Failed to create example file");
    file.write_all(input.as_bytes())
        .expect("Failed to write example input");

    eprintln!("---");
    eprintln!("Saved example to {}", example_path);
}

fn run_day(day: &str, example: bool) -> ! {
    let args: Vec<&str> = if example {
        vec!["run", "--release", "-p", day, "--", "--example"]
    } else {
        vec!["run", "--release", "-p", day]
    };

    let status = Command::new("cargo")
        .args(&args)
        .status()
        .expect("Failed to run cargo");
    std::process::exit(status.code().unwrap_or(1));
}

fn main() {
    let Cargo::Aoc(args) = Cargo::parse();

    // Extract day and example flag from Run or External
    let (day_str, example) = match &args.command {
        AocCommand::Run { day, example } => (day.clone(), *example),
        AocCommand::External(ext_args) => {
            let day = ext_args.first().cloned().unwrap_or_default();
            let example = ext_args.iter().any(|a| a == "--example");
            (day, example)
        }
        AocCommand::Bench { day } => (day.clone(), false),
        AocCommand::New { day } => (day.clone(), false),
    };

    let Some((day, day_num)) = parse_day(&day_str) else {
        AocArgs::command()
            .error(
                ErrorKind::InvalidSubcommand,
                format!("no such command '{day_str}'"),
            )
            .exit();
    };

    match args.command {
        AocCommand::Run { .. } | AocCommand::External(_) => {
            if example {
                ensure_example_input(day_num);
            }
            run_day(&day, example);
        }
        AocCommand::Bench { .. } => {
            let status = Command::new("cargo")
                .args(["run", "--release", "-p", &day, "--", "--bench"])
                .status()
                .expect("Failed to run benchmark");
            std::process::exit(status.code().unwrap_or(1));
        }
        AocCommand::New { .. } => {
            let status = Command::new("cargo")
                .args(["new", &day, "--vcs", "none"])
                .status()
                .expect("Failed to run cargo new");

            if !status.success() {
                std::process::exit(status.code().unwrap_or(1));
            }

            let cargo_path = format!("{day}/Cargo.toml");
            let mut cargo_toml =
                std::fs::read_to_string(&cargo_path).expect("Failed to read Cargo.toml");
            cargo_toml.push_str("aoc = { path = \"../common\" }\n");
            std::fs::write(&cargo_path, cargo_toml).expect("Failed to write Cargo.toml");

            let main_rs = format!("use aoc::*;\n\naoc_day!({day_num});\n");
            std::fs::write(format!("{day}/src/main.rs"), main_rs).expect("Failed to write main.rs");

            println!("Created {day}");
        }
    }
}
