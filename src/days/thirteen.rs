use std::collections::HashSet;
use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Transparent Origami";

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: usize,
    y: usize
}

impl FromStr for Point {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",")
            .ok_or_else(|| AocError::Misc(format!("Bad point string \"{}\"", s)))?;
        Ok(Point{
            x: x.parse()?,
            y: y.parse()?
        })
    }
}

#[derive(Debug)]
struct Paper {
    points: Vec<Point>,
    width: usize,
    height: usize
}

impl FromStr for Paper {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Paper {
            points: s.lines()
                .map(|l| l.parse())
                .collect::<Result<Vec<Point>, AocError>>()?,
            // This is wrong, the correct values would be the max of the points
            // in each direction. We don't need these to be correct until after
            // all the folding though, so it's easier/faster to be wrong here.
            width: 0,
            height: 0
        })
    }
}

#[derive(Debug)]
enum Fold {
    X(usize),
    Y(usize)
}

impl FromStr for Fold {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(" ").skip(2).next()
            .ok_or_else(|| AocError::Misc("Too few fold tokens".to_string()))?;
        let (axis, distance) = s.split_once("=")
            .ok_or_else(|| AocError::Misc("Bad fold string".to_string()))?;

        match axis {
            "x" => Ok(Fold::X(distance.parse()?)),
            "y" => Ok(Fold::Y(distance.parse()?)),
            _ => Err(AocError::Misc("Unknown fold axis".to_string()))
        }
    }
}

impl Paper {
    fn fold_x(&mut self, distance: usize) {
        for point in self.points.iter_mut() {
            if point.x > distance {
                point.x = distance - (point.x - distance);
            }
        }

        self.width = distance;
    }

    fn fold_y(&mut self, distance: usize) {
        for point in self.points.iter_mut() {
            if point.y > distance {
                point.y = distance - (point.y - distance);
            }
        }

        self.height = distance;
    }

    fn fold(&mut self, fold: &Fold) {
        match fold {
            Fold::X(distance) => self.fold_x(*distance),
            Fold::Y(distance) => self.fold_y(*distance)
        }
    }

    fn total_points(&self) -> usize {
        let mut uniq = HashSet::new();

        for point in &self.points {
            uniq.insert(point);
        }

        uniq.len()
    }

    fn draw(&self) -> String {
        let mut uniq = HashSet::new();
        for point in &self.points {
            uniq.insert(point);
        }

        // +1 here for the newlines
        let mut drawing = String::with_capacity((self.width + 1) * self.height);
        drawing.push('\n');

        for y in 0..self.height {
            for x in 0..self.width {
                if uniq.contains(&Point { x, y }) {
                    drawing.push('█');
                } else {
                    drawing.push(' ');
                }
            }

            if y < self.height - 1 {
                drawing.push('\n');
            }
        }

        drawing
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let (points, folds) = input.split_once("\n\n")
        .ok_or_else(|| AocError::Misc("Bad input".to_string()))?;
    let mut paper: Paper = points.parse()?;
    let fold: Fold = folds
        .lines()
        .next()
        .ok_or_else(|| AocError::Misc("No fold lines".to_string()))?
        .parse()?;

    paper.fold(&fold);

    Ok(paper.total_points().to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let (points, folds) = input.split_once("\n\n")
        .ok_or_else(|| AocError::Misc("Bad input".to_string()))?;
    let mut paper: Paper = points.parse()?;

    for line in folds.lines() {
        paper.fold(&line.parse()?);
    }

    // Add some extra spaces so my timing output lines up
    let mut drawing = paper.draw();
    drawing.push_str("            ");

    Ok(drawing.to_string())
}