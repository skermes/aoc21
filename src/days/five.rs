use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Hydrothermal Venture";

fn semisign(x: isize) -> isize {
    if x < 0 {
        -1
    } else if x > 0 {
        1
    } else {
        0
    }
}

#[derive(Debug, Copy, Clone)]
struct Point(isize, isize);

impl FromStr for Point {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").ok_or_else(|| AocError::Misc(format!("Invalid point \"{}\"", s)))?;
        Ok(Point(x.parse()?, y.parse()?))
    }
}

impl Point {
    fn x(&self) -> isize { self.0 }
    fn y(&self) -> isize { self.1 }
}

#[derive(Debug, Copy, Clone)]
struct Line {
    from: Point,
    to: Point
}

impl FromStr for Line {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once(" -> ").ok_or_else(|| AocError::Misc(format!("Invalid line \"{}\"", s)))?;
        Ok(Line { from: from.parse()?, to: to.parse()? })
    }
}

impl Line {
    fn is_axis_aligned(&self) -> bool {
        self.from.x() == self.to.x() || self.from.y() == self.to.y()
    }

    fn points(&self) -> LinePoints {
        LinePoints::new(self)
    }
}

#[derive(Debug)]
struct LinePoints {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    to: Point,
    done: bool
}

impl Iterator for LinePoints {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        self.x += self.dx;
        self.y += self.dy;

        if self.x == self.to.x() && self.y == self.to.y() {
            self.done = true;
        }

        Some(Point(self.x, self.y))
    }
}

impl LinePoints {
    fn new(line: &Line) -> Self {
        // This only works for vertical, horiztonal and 45 degree lines, but
        // thats all the problem asks for.
        let dx = semisign(line.to.x() - line.from.x());
        let dy = semisign(line.to.y() - line.from.y());

        LinePoints {
            dx, dy,
            // Subtract one tick here so that we can add it back during the
            // first iteration.
            x: line.from.x() - dx,
            y: line.from.y() - dy,
            to: line.to,
            done: false
        }
    }
}

#[derive(Debug)]
struct VentField2 {
    counts: [[u16; 1000]; 1000],
    overlaps: usize
}

impl VentField2 {
    fn add(&mut self, point: Point) {
        self.counts[point.x() as usize][point.y() as usize] += 1;

        if self.counts[point.x() as usize][point.y() as usize] == 2 {
            self.overlaps += 1;
        }
    }

    fn new() -> Self {
        VentField2 {
            counts: [[0; 1000]; 1000],
            overlaps: 0
        }
    }

    fn overlaps(&self) -> usize {
        self.overlaps
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let lines = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Line>, AocError>>()?;
    let mut field = VentField2::new();

    for line in lines {
        if line.is_axis_aligned() {
            for point in line.points() {
                field.add(point);
            }
        }
    }

    Ok(field.overlaps().to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let lines = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Line>, AocError>>()?;
    let mut field = VentField2::new();

    for line in lines {
        for point in line.points() {
            field.add(point);
        }
    }

    Ok(field.overlaps().to_string())
}