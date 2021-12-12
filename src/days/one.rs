use std::num::ParseIntError;
use crate::aoc_error::AocError;

pub const NAME: &str = "Sonar Sweep";

pub fn part_one(input: &str) -> Result<String, AocError> {
    let nums = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    let steps_up = nums.iter()
        .zip(nums.iter().skip(1))
        .filter(|(prev, next)| prev < next)
        .count();

    Ok(steps_up.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let nums = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    let window_sums: Vec<usize> = nums.iter()
        .zip(nums.iter().skip(1))
        .zip(nums.iter().skip(2))
        .map(|((x, y), z)| x + y + z)
        .collect();

    let steps_up = window_sums.iter()
        .zip(window_sums.iter().skip(1))
        .filter(|(prev, next)| prev < next)
        .count();

    Ok(steps_up.to_string())
}
