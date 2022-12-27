use clap::Parser;

/// Struct defining the arguments this program will accept from the command line
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The day of Advent of Code that we will be running the program for.
    #[arg(short, long)]
    pub day: u8,

    /// Each day has two parts: regular and advanced. Passing this flag means you want to run the advanced portion.
    #[arg(short, long)]
    pub advanced: bool
}