use std::{path::Path, fs::File, io::{BufReader, Read}};

use nom::{IResult, bytes::complete::{take_until, tag}, sequence::{terminated, separated_pair}, character::complete::{one_of}, combinator::{map_res, map_parser}, multi::many0};
use thiserror::Error;

use super::{DaySolution, DayError};

pub(super) struct Day2;

impl Day2 {
    fn calculate_score(input: Vec<RPSRound>) -> Result<u64, Day2Error> {
        return Ok(input.into_iter().map(|round| {
            let option_score = u64::from(RPS::get_value(&round.player_option));
            let round_result = RPS::get_result(&round);
            let result_score = u64::from(RPSResult::get_value(&round_result));
            
            return option_score + result_score
        }).sum())
    }
}

impl DaySolution for Day2 {
    const DAY: u8 = 2;

    const ADVANCED: bool = false;

    fn solve() -> super::DayResult {
        let input = Parser::parse(Self::build_input_path())
            .map_err(|e| {
                DayError::InternalDayError {
                    day: Self::DAY,
                    advanced: Self::ADVANCED,
                    source: Box::new(e)
                }
            })?;

        return Self::calculate_score(input)
        .map(|sum| { sum.to_string() })
        .map_err(|e| {
            DayError::InternalDayError {
                day: Self::DAY,
                advanced: Self::ADVANCED,
                source: Box::new(e)
            }
        })
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub(super) enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    pub fn from_char(symbol: &char) -> Option<Self> {
        match symbol {
            'A' | 'X' => Some(RPS::Rock),
            'B' | 'Y' => Some(RPS::Paper),
            'C' | 'Z' => Some(RPS::Scissors),
            _ => None
        }
    }

    pub fn to_player_symbol(symbol: &RPS) -> char {
        match symbol {
            RPS::Rock => 'X',
            RPS::Paper => 'Y',
            RPS::Scissors => 'Z'
        }
    }

    pub fn to_opponent_symbol(symbol: &RPS) -> char {
        match symbol {
            RPS::Rock => 'A',
            RPS::Paper => 'B',
            RPS::Scissors => 'C'
        }
    }

    pub fn get_value(symbol: &RPS) -> u32 {
        match symbol {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3
        }
    }

    pub fn get_result(round: &RPSRound) -> RPSResult {
        match (round.player_option, round.opponent_option) {
            (RPS::Rock, RPS::Scissors) | 
            (RPS::Paper, RPS::Rock) |
            (RPS::Scissors, RPS::Paper) => RPSResult::Win,
            (RPS::Rock, RPS::Paper) |
            (RPS::Paper, RPS::Scissors) |
            (RPS::Scissors, RPS::Rock) => RPSResult::Loss,
            (RPS::Rock, RPS::Rock) |
            (RPS::Paper, RPS::Paper) |
            (RPS::Scissors, RPS::Scissors) => RPSResult::Draw
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub(super) enum RPSResult {
    Win,
    Loss,
    Draw
}

impl RPSResult {
    pub fn get_value(result: &RPSResult) -> u32 {
        match result {
            RPSResult::Win => 6,
            RPSResult::Loss => 0,
            RPSResult::Draw => 3
        }
    }

    pub fn from_char(symbol: &char) -> Option<Self> {
        Some(match symbol {
            'X' => RPSResult::Loss,
            'Y' => RPSResult::Draw,
            'Z' => RPSResult::Win,
            _ => return None
        })
    }
}

pub(super) struct RPSRound {
    pub player_option: RPS,
    pub opponent_option: RPS
}

pub(super) struct Parser;

impl Parser {
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<Vec<RPSRound>, Day2Error> {
        let file_path = path.as_ref().to_string_lossy().to_string();
        let file_name = path.as_ref().file_name().map_or(String::new(), |file_name| { file_name.to_string_lossy().to_string() });

        match File::open(path) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut buffer = String::new();

                let _read_result = reader.read_to_string(&mut buffer).map_err(|e | {
                    Day2Error::FileToStringError {
                        file_name,
                        source: e
                    }
                })?;

                Self::parse_rounds(&buffer)
                    .map(|(_, rounds)| {
                        rounds
                    })
                    .map_err(|e| {
                        Day2Error::ParseRPSRoundsError {
                            source: e.to_owned()
                        }
                    })

            },
            Err(error) => {
                Err(Day2Error::FileOpenError {
                    file_path,
                    source: error
                })
            }
        }
    }

    fn parse_rounds(input: &str) -> IResult<&str, Vec<RPSRound>> {
        let round = take_until::<_, _, nom::error::Error<&str>>("\n");
        let round_trimmed = terminated(round, tag("\n"));
        let player_option = one_of("XYZ");
        let opponent_option = one_of("ABC");
        let round_delimiter = nom::character::complete::char(' ');
        let separated_options = separated_pair(opponent_option, round_delimiter, player_option);
        let separated_round = map_parser(round_trimmed, separated_options);
        let converted_round = map_res(separated_round, Self::round_from_chars);
        let mut rounds = many0(converted_round);

        rounds(input)
    }

    pub(super) fn rps_from_char(input: char) -> Result<RPS, nom::error::Error<char>> {
        RPS::from_char(&input)
        .ok_or(nom::error::Error{ 
            input: input, 
            code: nom::error::ErrorKind::Char 
        })
    }

    fn round_from_chars(input: (char, char)) -> Result<RPSRound, nom::error::Error<char>> {
        Ok(RPSRound {
            player_option: Self::rps_from_char(input.1)?,
            opponent_option: Self::rps_from_char(input.0)?
        })
    }
}

#[derive(Debug, Error)]
pub(super) enum Day2Error {
    #[error("Unable to read file: {file_name} to String")]
    FileToStringError {
        file_name: String,
        source: std::io::Error
    },
    #[error("Failed to open file from {file_path}")]
    FileOpenError {
        file_path: String,
        source: std::io::Error
    },
    #[error("Failed to parse rounds of RPS")]
    ParseRPSRoundsError {
        source: nom::Err<nom::error::Error<String>>
    },
    #[error("Failed to parse strategies")]
    ParseRPSStrategyError {
        source: nom::Err<nom::error::Error<String>>
    },
    #[error("Get() returned None when accessing map of options to outcomes with Key: {key:?}")]
    RPSMapAccessError {
        key: RPS
    },
    #[error("Get() returned None when accessing map of outcomes to options with Key: {key:?}")]
    RPSResultMapAccessError {
        key: RPSResult
    }
}