use std::collections::HashMap;
use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Extended Polymerization";

#[derive(Debug)]
struct Polymer {
    atoms: Vec<char>
}

impl FromStr for Polymer {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Polymer {
            atoms: s.chars().collect()
        })
    }
}

#[derive(Debug)]
struct Rules {
    rules: HashMap<(char, char), char>
}

impl FromStr for Rules {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = s.lines()
            .map(|l| {
                let (input, output) = l.split_once(" -> ")
                    .ok_or_else(|| AocError::Misc("Bad rule string".to_string()))?;
                let input: Vec<char> = input.chars().collect();
                let output: Vec<char> = output.chars().collect();

                Ok(((input[0], input[1]), output[0]))
            })
            .collect::<Result<HashMap<(char, char), char>, AocError>>()?;

        Ok(Rules { rules })
    }
}

#[derive(Debug, Clone)]
struct AtomCounts {
    counts: HashMap<char, usize>
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct PolymerSeed {
    left: char,
    right: char,
    depth: usize
}

#[derive(Debug)]
struct PolymerMemo {
    memo: HashMap<PolymerSeed, AtomCounts>
}

impl PolymerMemo {
    fn new() -> Self {
        PolymerMemo { memo: HashMap::new() }
    }

    fn get_counts(&mut self, seed: &PolymerSeed, rules: &Rules) -> AtomCounts {
        if let Some(counts) = self.memo.get(seed) {
            counts.clone()
        } else {
            let counts = seed.counts(rules, self);
            self.memo.insert(seed.clone(), counts.clone());
            counts
        }
    }
}

impl AtomCounts {
    fn merge(&mut self, other: &AtomCounts) {
        for (key, value) in other.counts.iter() {
            if let Some(n) = self.counts.get_mut(&key) {
                *n += value;
            } else {
                self.counts.insert(*key, *value);
            }
        }
    }

    fn sub(&mut self, c: char) {
        if let Some(n) = self.counts.get_mut(&c) {
            *n -= 1;
        }
    }

    fn min(&self) -> usize {
        *self.counts.values().min().unwrap_or(&0)
    }

    fn max(&self) -> usize {
        *self.counts.values().max().unwrap_or(&0)
    }
}

impl PolymerSeed {
    fn counts(&self, rules: &Rules, memo: &mut PolymerMemo) -> AtomCounts {
        if self.depth == 0 {
            if self.left == self.right {
                AtomCounts { counts: HashMap::from([(self.left, 2)]) }
            } else {
                AtomCounts { counts: HashMap::from([(self.left, 1), (self.right, 1)]) }
            }
        } else {
            let middle = rules.get(self.left, self.right);
            let left = PolymerSeed {
                left: self.left,
                right: middle,
                depth: self.depth - 1
            };
            let right = PolymerSeed {
                left: middle,
                right: self.right,
                depth: self.depth -1
            };

            let mut counts = memo.get_counts(&left, rules);
            counts.merge(&memo.get_counts(&right, rules));
            // Both the left and right seeds will count the middle char, so
            // sub it back out.
            counts.sub(middle);

            counts
        }
    }
}

impl Rules {
    fn get(&self, left: char, right: char) -> char {
        // Input is "complete", all possible pairs of characters are in the rules.
        *self.rules.get(&(left, right)).unwrap()
    }
}

impl Polymer {
    fn counts_after_iterations(&self, rules: &Rules, iterations: usize) -> AtomCounts {
        let mut memo = PolymerMemo::new();
        let mut counts = AtomCounts { counts: HashMap::new() };

        for window in self.atoms.windows(2) {
            let seed = PolymerSeed {
                left: window[0],
                right: window[1],
                depth: iterations
            };
            counts.merge(&memo.get_counts(&seed, rules));
        }

        for c in self.atoms.iter().skip(1).take(self.atoms.len() - 2) {
            // Each window above double counts the middle characters once.
            counts.sub(*c)
        }

        counts
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let (polymer, rules) = input.split_once("\n\n")
        .ok_or_else(|| AocError::Misc("No blank line input".to_string()))?;
    let polymer: Polymer = polymer.parse()?;
    let rules: Rules = rules.parse()?;

    let counts = polymer.counts_after_iterations(&rules, 10);
    Ok((counts.max() - counts.min()).to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let (polymer, rules) = input.split_once("\n\n")
        .ok_or_else(|| AocError::Misc("No blank line input".to_string()))?;
    let polymer: Polymer = polymer.parse()?;
    let rules: Rules = rules.parse()?;

    let counts = polymer.counts_after_iterations(&rules, 40);
    Ok((counts.max() - counts.min()).to_string())
}