use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Dive!";

#[derive(Debug, Copy, Clone)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize)
}
use Command::*;

impl FromStr for Command {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("forward") {
            Ok(Forward(s[8..].parse()?))
        } else if s.starts_with("up") {
            Ok(Up(s[3..].parse()?))
        } else if s.starts_with("down") {
            Ok(Down(s[5..].parse()?))
        } else {
            Err(AocError::Misc(format!("Invalid command \"{}\"", s)))
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Position {
    horizontal: usize,
    depth: usize,
    aim: usize
}

impl Position {
    fn update_p1(&self, command: Command) -> Self {
        match command {
            Forward(d) => Position {
                horizontal: self.horizontal + d,
                depth: self.depth,
                aim: self.aim
            },
            Down(d) => Position {
                horizontal: self.horizontal,
                depth: self.depth + d,
                aim: self.aim
            },
            Up(d) => Position {
                horizontal: self.horizontal,
                depth: self.depth - d,
                aim: self.aim
            }
        }
    }

    fn update_p2(&self, command: Command) -> Self {
        match command {
            Down(d) => Position {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim + d
            },
            Up(d) => Position {
                horizontal: self.horizontal,
                depth: self.depth,
                aim: self.aim - d
            },
            Forward(d) => Position {
                horizontal: self.horizontal + d,
                depth: self.depth + self.aim * d,
                aim: self.aim
            }
        }
    }

    fn product(&self) -> usize {
        self.horizontal * self.depth
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let answer = input
        .lines()
        .map(|l| l.parse())
        .fold(
            Ok(Position { horizontal: 0, depth: 0, aim: 0 }),
            |p: Result<Position, AocError>, c| Ok(p?.update_p1(c?))
        )?
        .product();

    Ok(answer.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let answer = input
        .lines()
        .map(|l| l.parse())
        .fold(
            Ok(Position { horizontal: 0, depth: 0, aim: 0 }),
            |p: Result<Position, AocError>, c| Ok(p?.update_p2(c?))
        )?
        .product();

    Ok(answer.to_string())
}