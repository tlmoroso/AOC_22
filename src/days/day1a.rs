use crate::days::{
    DaySolution, DayResult, DayError, 
    day1::{
        Parser, Day1Error
    }
};

pub struct Day1a;

impl Day1a {
    fn find_3_top_calories(input: Vec<Vec<u32>>) -> Result<u64, Day1Error> {
    const NUM_ELVES: usize = 3; 

        let mut calorie_sums = input.into_iter().enumerate().map(|(index, calorie_list)| {
            let sum = calorie_list.into_iter().map(|value| { u64::from(value) }).sum();
            println!("Elf #{} Total Calories = {}", index, sum);
            return sum
        }).collect::<Vec<u64>>();

        calorie_sums.sort();
        let (_head, top_3) = calorie_sums.split_at(calorie_sums.len() - NUM_ELVES);

        let sum = top_3.into_iter().sum();
        
        return if sum == 0 {
            Err(Day1Error::EmptyInputError{})
        } else {
            Ok(sum)
        }
    }
}

impl DaySolution for Day1a {
    const DAY: u8 = 1;
    const ADVANCED: bool = true;

    fn solve() -> DayResult {
        let input = Parser::parse(Self::build_input_path())
        .map_err(|e| { 
            DayError::InternalDayError {
                day: Self::DAY,
                advanced: Self::ADVANCED,
                source: Box::new(e)
            }
        })?;

        return Self::find_3_top_calories(input)
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