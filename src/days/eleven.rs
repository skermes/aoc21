use std::collections::VecDeque;
use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Dumbo Octopus";

const SIZE: usize = 10;

#[derive(Debug)]
struct Point(usize, usize);

impl Point {
    fn neighbors(&self) -> Vec<Self> {
        let up = self.1 > 0;
        let down = self.1 < SIZE - 1;
        let left = self.0 > 0;
        let right = self.0 < SIZE - 1;

        let mut points = Vec::with_capacity(8);

        if up && left    { points.push(Point(self.0 - 1, self.1 - 1)); }
        if up            { points.push(Point(self.0,     self.1 - 1)); }
        if up && right   { points.push(Point(self.0 + 1, self.1 - 1)); }
        if left          { points.push(Point(self.0 - 1, self.1    )); }
        if right         { points.push(Point(self.0 + 1, self.1    )); }
        if down && left  { points.push(Point(self.0 - 1, self.1 + 1)); }
        if down          { points.push(Point(self.0,     self.1 + 1)); }
        if down && right { points.push(Point(self.0 + 1, self.1 + 1)); }

        points
    }

    fn index(&self) -> usize {
        self.1 * SIZE + self.0
    }

    fn from_index(i: usize) -> Self {
        Point(i % SIZE, i / SIZE)
    }
}

#[derive(Debug)]
struct Octopuses {
    grid: Vec<u32>
}

impl FromStr for Octopuses {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Octopuses {
            grid: s.lines()
                .flat_map(|l| l.chars())
                .map(|c| c.to_digit(10).ok_or_else(|| AocError::Misc(format!("Bad char '{}'", c))))
                .collect::<Result<Vec<u32>, AocError>>()?
        })
    }
}

impl Octopuses {
    fn step(&mut self) -> usize {
        let mut to_flash = VecDeque::new();

        for (i, octopus) in self.grid.iter_mut().enumerate() {
            *octopus += 1;

            if *octopus == 10 { to_flash.push_back(Point::from_index(i)); }
        }

        while !to_flash.is_empty() {
            let next = to_flash.pop_front().unwrap();

            for point in next.neighbors() {
                self.grid[point.index()] += 1;
                if self.grid[point.index()] == 10 { to_flash.push_back(point); }
            }
        }

        let flashes = self.grid.iter().filter(|&&o| o > 9).count();

        for octopus in self.grid.iter_mut() {
            if *octopus > 9 { *octopus = 0; }
        }

        flashes
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let mut octos: Octopuses = input.parse()?;
    let total_flashes: usize = (0..100)
        .map(|_| octos.step())
        .sum();

    Ok(total_flashes.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let mut octos: Octopuses = input.parse()?;

    let synchro_flash = (1..usize::MAX)
        .skip_while(|_| octos.step() < 100)
        .next()
        .unwrap();

    Ok(synchro_flash.to_string())
}