use std::num::ParseIntError;
use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Giant Squid";

#[derive(Debug, Copy, Clone)]
enum BingoNumber {
    Matched,
    Unmatched(usize)
}
use BingoNumber::*;

impl From<&BingoNumber> for usize {
    fn from(n: &BingoNumber) -> Self {
        match n {
            Matched => 0,
            Unmatched(n) => *n
        }
    }
}

impl BingoNumber {
    fn is_match(&self) -> bool {
        match self {
            Matched => true,
            Unmatched(_) => false
        }
    }
}

#[derive(Debug)]
struct Board {
    numbers: Vec<BingoNumber>
}

impl FromStr for Board {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Board {
            numbers: s
                .split_whitespace()
                .map(|s| s.parse())
                .map(|n| Ok(Unmatched(n?)))
                .collect::<Result<Vec<BingoNumber>, ParseIntError>>()?
        })
    }
}

const SLICES: [[usize; 5]; 10] = [
    [0, 1, 2, 3, 4],
    [5, 6, 7, 8, 9],
    [10, 11, 12, 13, 14],
    [15, 16, 17, 18, 19],
    [20, 21, 22, 23, 24],
    [0, 5, 10, 15, 20],
    [1, 6, 11, 16, 21],
    [2, 7, 12, 17, 22],
    [3, 8, 13, 18, 23],
    [4, 9, 14, 19, 24]
];

impl Board {
    fn mark(&mut self, number: usize) {
        for n in self.numbers.iter_mut() {
            if let Unmatched(x) = n {
                if x == &number {
                    *n = Matched;
                    break;
                }
            }
        }
    }

    fn has_won(&self) -> bool {
        SLICES.iter()
            .map(|slice| slice.iter()
                            .map(|i| self.numbers[*i])
                            .all(|n| n.is_match()))
            .any(|b| b)
    }

    fn score(&self) -> usize {
        self.numbers
            .iter()
            .map(|n| usize::from(n))
            .sum()
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let mut components = input.split("\n\n");

    let drawn_numbers = components.next()
        .ok_or_else(|| AocError::Misc("No drawn numbers in input".to_string()))?
        .split(",")
        .map(|s| s.parse())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    let mut boards = components
        .map(|s| s.parse())
        .collect::<Result<Vec<Board>, AocError>>()?;

    for number in drawn_numbers {
        for board in boards.iter_mut() {
            board.mark(number);
            if board.has_won() {
                return Ok((board.score() * number).to_string())
            }
        }
    }

    Ok("No winning board".to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let mut components = input.split("\n\n");

    let drawn_numbers = components.next()
        .ok_or_else(|| AocError::Misc("No drawn numbers in input".to_string()))?
        .split(",")
        .map(|s| s.parse())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;

    let mut boards = components
        .map(|s| s.parse())
        .collect::<Result<Vec<Board>, AocError>>()?;

    let mut wins = 0;
    let total = boards.len();

    for number in drawn_numbers {
        for board in boards.iter_mut() {
            if board.has_won() {
                continue;
            }

            board.mark(number);
            if board.has_won() {
                wins += 1;
                if wins == total {
                    return Ok((board.score() * number).to_string())
                }
            }
        }
    }

    Ok("Not all boards win".to_string())
}