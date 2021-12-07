use std::num::ParseIntError;
use crate::aoc_error::AocError;

pub const NAME: &str = "The Treachery of Whales";

// Like regular median except it returns the higher of the middle pair when the
// list has an even number of items.
// Input must be sorted.
fn crab_median(positions: &[usize]) -> usize {
    positions[positions.len() / 2]
}

// Like regular mean but truncates the division instead of using floats or
// rounding properly.
fn crab_mean(positions: &[usize]) -> usize {
    let sum: usize = positions.iter().sum();
    sum / positions.len()
}

fn fuel_cost_p1(position: usize, target: usize) -> usize {
    if position < target {
        target - position
    } else {
        position - target
    }
}

// Fuel costs in part 2 are triangle numbers
fn fuel_cost_p2(position: usize, target: usize) -> usize {
    let distance = fuel_cost_p1(position, target);
    (distance * (distance + 1)) / 2
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let mut positions = input
        .split(",")
        .map(|s| s.parse())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;
    positions.sort();

    let best_gather_spot = crab_median(&positions);

    let total_fuel: usize = positions
        .iter()
        .map(|p| fuel_cost_p1(*p, best_gather_spot))
        .sum();

    Ok(total_fuel.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let positions = input
        .split(",")
        .map(|s| s.parse())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    let low_candidate = crab_mean(&positions);
    let high_candidate = low_candidate + 1;

    let low_fuel: usize = positions
        .iter()
        .map(|p| fuel_cost_p2(*p, low_candidate))
        .sum();
    let high_fuel: usize = positions
        .iter()
        .map(|p| fuel_cost_p2(*p, high_candidate))
        .sum();

    let total_fuel = if low_fuel < high_fuel { low_fuel } else { high_fuel };

    Ok(total_fuel.to_string())
}