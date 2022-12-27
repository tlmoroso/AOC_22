pub mod day1;
pub mod day1a;

use thiserror::Error;

use self::{day1::Day1, day1a::Day1a};

pub type DayResult = Result<String, DayError>;

pub trait DaySolution {
    const DAY: u8;
    const ADVANCED: bool;

    fn solve() -> DayResult;

    fn build_input_path() -> String {
        let advanced_suffix = if Self::ADVANCED { "a" } else { "" };
        let input_path = format!("./input/{}{}.txt", Self::DAY, advanced_suffix);
        return input_path
    }
}



pub fn solve_day(day: u8, advanced: bool) -> DayResult {
    match (day, advanced) {
        (1, false) => Day1::solve(),
        (1, true) => Day1a::solve(),
        _ => Err(DayError::InvalidDay { day })
    }
}



#[derive(Error, Debug)]
pub enum DayError {
    #[error("Invalid Day: {day}")]
    InvalidDay { day: u8 },
    #[error("Error returned from internal solver for Day {day}{}", if *advanced {"a"} else {""})]
    InternalDayError {
        day: u8,
        advanced: bool,
        source: Box<dyn std::error::Error>
    }
}