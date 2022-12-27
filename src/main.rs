use aoc_22::{parser::args::Args, days::solve_day};
use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("Running Day {}{}...", args.day, if args.advanced {"a"} else {""});
    let result = solve_day(args.day, args.advanced);
    println!("Result: {:#?}", result)
}
