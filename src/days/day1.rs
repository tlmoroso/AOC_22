use std::{path::Path, fs::File, io::{BufReader, Read}, str::FromStr};
use thiserror::Error;
use nom::{IResult, bytes::complete::{take_until, tag}, multi::many0, combinator::{map_res}, sequence::{terminated, pair}, character::complete::digit0};

use crate::days::DaySolution;
use crate::days::DayResult;

use super::DayError;

pub struct Day1;

impl Day1 {
    fn find_max_calories(input: Vec<Vec<u32>>) -> Result<u64, Day1Error> {
        let calorie_sums = input.into_iter().enumerate().map(|(index, calorie_list)| {
            let sum = calorie_list.into_iter().map(|value| { u64::from(value) }).sum();
            println!("Elf #{} Total Calories = {}", index, sum);
            return sum
        }).collect::<Vec<u64>>();

        return calorie_sums.into_iter().max().ok_or(Day1Error::EmptyInputError{})
    }
}

impl DaySolution for Day1 {
    const DAY: u8 = 1;
    const ADVANCED: bool = false;

    fn solve() -> DayResult {
        let input = Parser::parse(Self::build_input_path())
        .map_err(|e| { 
            DayError::InternalDayError {
                day: Self::DAY,
                advanced: Self::ADVANCED,
                source: Box::new(e)
            }
        })?;

        return Self::find_max_calories(input)
            .map(|max| { max.to_string() })
            .map_err(|e| {
                DayError::InternalDayError {
                    day: Self::DAY,
                    advanced: Self::ADVANCED,
                    source: Box::new(e)
                }
            })
    }
}

struct Parser;

impl Parser {
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<Vec<Vec<u32>>, Day1Error> {
        let file_path = path.as_ref().to_string_lossy().to_string();
        let file_name = path.as_ref().file_name().map_or(String::new(), |file_name| { file_name.to_string_lossy().to_string() });

        match File::open(path) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut buffer = String::new();

                let _read_result = reader.read_to_string(&mut buffer).map_err(|e | {
                    Day1Error::FileToStringError {
                        file_name,
                        source: e
                    }
                })?;

                Self::parse_elves(&buffer)
                    .map(|(_, lists)| { lists })
                    .map_err(|e| { 
                        Day1Error::ParseInputError {
                            input: file_path,
                            source: e.to_owned()
                        }
                    })
            },
            Err(error) => {
                Err(Day1Error::FileOpenError {
                    file_path,
                    source: error
                })
            }
        }
    }

    fn parse_elves(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
        let elf_calories = map_res(terminated(take_until("\n\n"), tag("\n\n")), Self::parse_elf);

        many0(elf_calories)(input)
            .map(|(elf_input, elf_list)| {
                let calorie_lists = elf_list.into_iter().map(|(_, calorie_list)| { calorie_list }).collect::<Vec<Vec<u32>>>();
                (elf_input, calorie_lists)
            })
    }

    fn parse_elf(input: &str) -> IResult<&str, Vec<u32>> {
        println!("Converting {} into Vec<u32>", input);
        
        let single_value = take_until("\n");
        let value_without_newline = terminated(single_value, tag("\n"));
        let value_as_u32 = map_res(value_without_newline, Self::u32_from_str);
        let list_of_values = many0(value_as_u32);
        let final_number = digit0;
        let final_number_as_u32 = map_res(final_number, Self::u32_from_str);
        let get_final_number = pair(list_of_values, final_number_as_u32);
        let mut appended_final_number = map_res(get_final_number, |(mut list, last_number)| { list.push(last_number); return Ok::<Vec<u32>, nom::Err<nom::error::Error<&str>>>(list) });

        appended_final_number(input)
    }

    fn u32_from_str(input: &str) -> Result<u32, std::num::ParseIntError> {
        let uinteger = u32::from_str(input);
        println!("Converting {} to u32", input);
        return uinteger
    }
}

#[derive(Error, Debug)]
enum Day1Error {
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
    #[error("Failed to Parse Elves' Calories from input={input}")]
    ParseInputError {
        input: String,
        source: nom::Err<nom::error::Error<String>>
    },
    #[error("Input was somehow empty")]
    EmptyInputError {}
}