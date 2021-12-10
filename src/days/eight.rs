use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Seven Segment Search";

const A: usize = 0b0000001;
const B: usize = 0b0000010;
const C: usize = 0b0000100;
const D: usize = 0b0001000;
const E: usize = 0b0010000;
const F: usize = 0b0100000;
const G: usize = 0b1000000;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Segments(usize);

impl FromStr for Segments {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.chars()
            .map(|c| match c {
                'a' => Ok(A),
                'b' => Ok(B),
                'c' => Ok(C),
                'd' => Ok(D),
                'e' => Ok(E),
                'f' => Ok(F),
                'g' => Ok(G),
                _ => Err(AocError::Misc(format!("Invalid segment \"{}\"", c)))
            })
            .fold(Ok(0), |acc, x| match (acc, x) {
                (Ok(acc), Ok(x)) => Ok(acc | x),
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e)
            })?;
        Ok(Segments(value))
    }
}

impl Segments {
    fn len(&self) -> u32 {
        self.0.count_ones()
    }

    fn intersection(&self, other: &Segments) -> Segments {
        Segments(self.0 & other.0)
    }

    fn is_1_or_4_or_7_or_8(&self) -> bool {
        self.len() == 2 || self.len() == 3 || self.len() == 4 || self.len() == 7
    }
}

#[derive(Debug)]
struct SegmentDictionary {
    digits: Vec<Segments>
}

impl FromStr for SegmentDictionary {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SegmentDictionary {
            digits: s.split(" ")
                .map(|s| s.parse())
                .collect::<Result<Vec<Segments>, AocError>>()?
        })
    }
}

impl SegmentDictionary {
    // Unwraps in digit finders are safe as long as we assume the dictionary
    // is well-formed - it has one of every digit.

    fn one(&self) -> &Segments {
        self.digits
            .iter()
            .filter(|s| s.len() == 2)
            .next()
            .unwrap()
    }

    fn four(&self) -> &Segments {
        self.digits
            .iter()
            .filter(|s| s.len() == 4)
            .next()
            .unwrap()
    }

    fn digit(&self, segment: &Segments) -> Result<usize, AocError> {
        match segment.len() {
            2 => Ok(1),
            3 => Ok(7),
            4 => Ok(4),
            7 => Ok(8),
            5 => {
                if self.four().intersection(segment).len() == 2 {
                    Ok(2)
                } else if self.one().intersection(segment).len() == 2{
                    Ok(3)
                } else {
                    Ok(5)
                }
            },
            6 => {
                if self.four().intersection(segment).len() == 4 {
                    Ok(9)
                } else if self.one().intersection(segment).len() == 2 {
                    Ok(0)
                } else {
                    Ok(6)
                }
            },
            _ => Err(AocError::Misc("Bad digit".to_string()))
        }
    }
}

#[derive(Debug)]
struct KrangledDisplay {
    signals: SegmentDictionary,
    outputs: Vec<Segments>
}

impl FromStr for KrangledDisplay {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signals, outputs) = s.split_once(" | ")
            .ok_or_else(|| AocError::Misc(format!("Invalid display string \"{}\"", s)))?;

        Ok(KrangledDisplay {
            signals: signals.parse()?,
            outputs: outputs.split(" ")
                .map(|s| s.parse())
                .collect::<Result<Vec<Segments>, AocError>>()?
        })
    }
}

impl KrangledDisplay {
    fn output(&self) -> Result<usize, AocError> {
        Ok(self.signals.digit(&self.outputs[0])? * 1000 +
           self.signals.digit(&self.outputs[1])? * 100 +
           self.signals.digit(&self.outputs[2])? * 10 +
           self.signals.digit(&self.outputs[3])?)
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let count = input.lines()
        .map(|l| l.parse::<KrangledDisplay>())
        .fold(Ok(0), |sum: Result<usize, AocError>, display| {
            let specials = display?.outputs
                .iter()
                .filter(|s| s.is_1_or_4_or_7_or_8())
                .count();
            Ok(sum? + specials)
        })?;

    Ok(count.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let sum = input.lines()
        .map(|l| l.parse::<KrangledDisplay>())
        .fold(Ok(0), |sum: Result<usize, AocError>, display| {
            Ok(sum? + display?.output()?)
        })?;

    Ok(sum.to_string())
}