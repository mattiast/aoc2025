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

    #[arg(short, long, help = "Path to input file (defaults to inputs/dayXX.txt)")]
    input: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    if args.day < 1 || args.day > 25 {
        eprintln!("Error: Day must be between 1 and 25");
        std::process::exit(1);
    }

    if args.part != 1 && args.part != 2 {
        eprintln!("Error: Part must be 1 or 2");
        std::process::exit(1);
    }

    let solution = match days::get_solution(args.day) {
        Some(s) => s,
        None => {
            eprintln!("Error: Day {} is not yet implemented", args.day);
            std::process::exit(1);
        }
    };

    let input_path = args.input.unwrap_or_else(|| {
        PathBuf::from(format!("inputs/day{:02}.txt", args.day))
    });

    let input = match fs::read_to_string(&input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input file {:?}: {}", input_path, e);
            std::process::exit(1);
        }
    };

    let result = match args.part {
        1 => solution.part1(&input),
        2 => solution.part2(&input),
        _ => unreachable!(),
    };

    println!("{}", result);
}
