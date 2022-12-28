use std::{collections::HashMap, path::Path, io::{BufReader, Read}, fs::File};
use nom::{IResult, bytes::complete::{take_until, tag}, sequence::{terminated, separated_pair}, character::complete::one_of, combinator::{map_parser, map_res}, multi::many0};

use crate::days::day2::{RPS, RPSResult, Day2Error};

use super::{day2::Parser, DaySolution, DayError};

pub struct Day2a;

impl Day2a {
    pub(super) fn calculate_score(input: Vec<StrategyGuide>) -> Result<u64, Day2Error> {
        let result_map: HashMap<RPS, HashMap<RPSResult, RPS>> = HashMap::from(
            [
                (
                    RPS::Rock, 
                    HashMap::from([
                        (RPSResult::Win, RPS::Paper),
                        (RPSResult::Draw, RPS::Rock),
                        (RPSResult::Loss, RPS::Scissors)
                    ])
                ),
                (
                    RPS::Paper,
                    HashMap::from([
                        (RPSResult::Win, RPS::Scissors),
                        (RPSResult::Draw, RPS::Paper),
                        (RPSResult::Loss, RPS::Rock)
                    ])
                ),
                (
                    RPS::Scissors,
                    HashMap::from([
                        (RPSResult::Win, RPS::Rock),
                        (RPSResult::Draw, RPS::Scissors),
                        (RPSResult::Loss, RPS::Paper)
                    ])
                )
            ]
        );

        return input.into_iter()
            .map(|guide| {
                let outcome = guide.outcome;
                let player_option = result_map
                    .get(&guide.opponent_option)
                    .ok_or(Day2Error::RPSMapAccessError{ key: guide.opponent_option })?
                    .get(&outcome)
                    .ok_or(Day2Error::RPSResultMapAccessError{ key: outcome })?;

                let outcome_points = u64::from(RPSResult::get_value(&outcome));
                let option_points = u64::from(RPS::get_value(player_option));

                return Ok(option_points + outcome_points)
            })
            .fold(Ok(0u64), |sum, points| {
                if let Ok(total_points) = sum {
                    if let Ok(round_points) = points {
                        return Ok(total_points + round_points)
                    } else {
                        return points
                    }
                } else {
                    return sum
                }
            });
    }
}

impl DaySolution for Day2a {
    const DAY: u8 = 2;

    const ADVANCED: bool = true;

    fn solve() -> super::DayResult {
        let input = Parser::parse_strategy_guide(Self::build_input_path())
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

pub(super) struct StrategyGuide {
    pub opponent_option: RPS,
    pub outcome: RPSResult
}

impl Parser {
    pub fn parse_strategy_guide<P: AsRef<Path>>(path: P) -> Result<Vec<StrategyGuide>, Day2Error> {
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

                Self::parse_strategies(&buffer)
                    .map(|(_, strategies)| {
                        strategies
                    })
                    .map_err(|e| {
                        Day2Error::ParseRPSStrategyError {
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

    fn parse_strategies(input: &str) -> IResult<&str, Vec<StrategyGuide>> {
        let round = take_until::<_, _, nom::error::Error<&str>>("\n");
        let round_trimmed = terminated(round, tag("\n"));
        let outcome = one_of("XYZ");
        let opponent_option = one_of("ABC");
        let round_delimiter = nom::character::complete::char(' ');
        let option_and_outcome = separated_pair(opponent_option, round_delimiter, outcome);
        let separated_round = map_parser(round_trimmed, option_and_outcome);
        let converted_round = map_res(separated_round, Self::strategy_from_chars);
        let mut rounds = many0(converted_round);

        rounds(input)
    }

    fn strategy_from_chars(input: (char, char)) -> Result<StrategyGuide, nom::error::Error<char>> {
        Ok(StrategyGuide {
            opponent_option: Self::rps_from_char(input.0)?,
            outcome: Self::outcome_from_char(input.1)?
        })
    }

    fn outcome_from_char(input: char) -> Result<RPSResult, nom::error::Error<char>> {
        RPSResult::from_char(&input)
        .ok_or(nom::error::Error{ 
            input: input, 
            code: nom::error::ErrorKind::Char 
        })
    }
}