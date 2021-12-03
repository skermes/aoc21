use std::iter::repeat;
use crate::aoc_error::AocError;

pub const NAME: &str = "Binary Diagnostic";

fn find_one_counts(lines: &[&str]) -> Vec<usize> {
    if lines.len() == 0 {
        return Vec::new();
    }

    let mut counts = Vec::with_capacity(lines[0].len());
    counts.extend(repeat(0).take(lines[0].len()));

    lines
        .iter()
        .fold(counts, |mut counts, line| {
            let ones = line.chars()
                .enumerate()
                .filter(|(_, bit)| bit == &'1')
                .map(|(position, _)| position);

            for position in ones {
                counts[position] += 1;
            }

            counts
        })
}

fn find_life_support_component(input: &str, gte: &str, lt: &str) -> Result<usize, AocError> {
    let mut candidates: Vec<&str> = input.lines().collect();
    let mut index = 0;

    loop {
        if candidates.len() == 1 {
            return Ok(usize::from_str_radix(candidates[0], 2).unwrap());
        }

        let bit_counts = find_one_counts(&candidates);
        if index >= bit_counts.len() {
            return Err(AocError::Misc("Input doesn't collapse to single component".to_string()));
        }

        let keep = if bit_counts[index] >= (candidates.len() - bit_counts[index]) { gte } else { lt };
        candidates = candidates.iter()
            .filter(|c| &c[index..(index + 1)] == keep)
            .map(|&c| c)
            .collect();

        index += 1;
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let total_nums = input.lines().count();

    let set_ones = find_one_counts(&input.lines().collect::<Vec<&str>>());

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for position in set_ones {
        if position > (total_nums / 2) {
            gamma_rate += 1;
        } else {
            epsilon_rate += 1;
        }

        gamma_rate <<= 1;
        epsilon_rate <<= 1;
    }

    // Undo extra shift after the last iteration
    gamma_rate >>= 1;
    epsilon_rate >>= 1;

    Ok((epsilon_rate * gamma_rate).to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let o2 = find_life_support_component(input, "1", "0")?;
    let co2 = find_life_support_component(input, "0", "1")?;

    Ok((o2 * co2).to_string())
}