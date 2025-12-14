use anyhow::bail;
use aoc2025::days;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "aoc2025")]
#[command(about = "Advent of Code 2025 Solutions", long_about = None)]
struct Args {
    #[arg(help = "Day number (1-25)")]
    day: u8,

    #[arg(help = "Part number (1 or 2)")]
    part: u8,

    #[arg(
        short,
        long,
        help = "Path to input file (defaults to inputs/dayXX.txt)"
    )]
    input: Option<PathBuf>,
}

enum Part {
    One,
    Two,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let part = match args.part {
        1 => Part::One,
        2 => Part::Two,
        _ => {
            bail!("Error: Part must be 1 or 2");
        }
    };

    let solution = days::get_solution(args.day).ok_or(anyhow::anyhow!(
        "Error: Day {} is not implemented",
        args.day
    ))?;

    let input_path = args
        .input
        .unwrap_or_else(|| PathBuf::from(format!("inputs/day{:02}.txt", args.day)));

    let input = fs::read_to_string(&input_path)?;

    let result = match part {
        Part::One => solution.part1(&input),
        Part::Two => solution.part2(&input),
    };
    match result {
        Ok(res) => println!("{}", res),
        Err(e) => {
            bail!("Error executing solution: {}", e);
        }
    };

    Ok(())
}
