use aoc_22::parser::args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("Running Day {}...", args.day)
}
