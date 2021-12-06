use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Lanternfish";

#[derive(Debug)]
struct LanternfishPopulation {
    // The number of fish at each stage of development
    timers: [usize; 9]
}

impl FromStr for LanternfishPopulation {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut timers = [0; 9];
        for time in s.split(",").map(|s| s.parse::<usize>()) {
            timers[time?] += 1;
        }

        Ok(LanternfishPopulation { timers })
    }
}

impl LanternfishPopulation {
    fn step(&mut self) {
        let spawning = self.timers[0];
        for i in 1..9 {
            self.timers[i - 1] = self.timers[i];
        }
        self.timers[8] = spawning;
        self.timers[6] += spawning;
    }

    fn total_pop(&self) -> usize {
        self.timers.iter().sum()
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let mut pop: LanternfishPopulation = input.parse()?;

    for _ in 0..80 {
        pop.step();
    }

    Ok(pop.total_pop().to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let mut pop: LanternfishPopulation = input.parse()?;

    for _ in 0..256 {
        pop.step();
    }

    Ok(pop.total_pop().to_string())
}