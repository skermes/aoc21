use crate::aoc_error::AocError;

pub const NAME: &str = "Trick Shot";

#[derive(Debug, Copy, Clone)]
struct Point(isize, isize);

impl Point {
    fn x(&self) -> isize { self.0 }
    fn y(&self) -> isize { self.1 }
}

#[derive(Debug, Copy, Clone)]
struct Area(Point, Point);

impl Area {
    fn left(&self) -> isize {
        if self.0.x() < self.1.x() { self.0.x() } else { self.1.x() }
    }

    fn right(&self) -> isize {
        if self.0.x() > self.1.x() { self.0.x() } else { self.1.x() }
    }

    fn top(&self) -> isize {
        if self.0.y() > self.1.y() { self.0.y() } else { self.1.y() }
    }

    fn bottom(&self) -> isize {
        if self.0.y() < self.1.y() { self.0.y() } else { self.1.y() }
    }

    fn height(&self) -> isize {
        self.top() - self.bottom()
    }

    fn contains(&self, point: &Point) -> bool {
        self.left() <= point.x() && self.right() >= point.x() &&
        self.top() >= point.y() && self.bottom() <= point.y()
    }
}

#[derive(Debug, Copy, Clone)]
struct Arc {
    vx: isize,
    vy: isize,
    loc: Point
}

impl Arc {
    fn step(&self) -> Self {
        Arc {
            vx: if self.vx < 0 {
                    self.vx + 1
                } else if self.vx > 0 {
                    self.vx - 1
                } else {
                    self.vx
                },
            vy: self.vy - 1,
            loc: Point(self.loc.x() + self.vx, self.loc.y() + self.vy)
        }
    }

    fn passes_through(&self, area: &Area) -> bool {
        // There are other false conditions I could check, but this will cover
        // it eventually

        if self.vy < 0 && self.loc.y() < area.bottom() {
            false
        } else if area.contains(&self.loc) {
            true
        } else {
            self.step().passes_through(area)
        }
    }

    fn max_height(&self) -> isize {
        // Adding up consecutive integers is always the nth triangle number
        (self.vy * (self.vy + 1)) / 2
    }
}

// target area: x=282..314, y=-80..-45
const INPUT: Area = Area(Point(282, -80), Point(314, -45));
// const INPUT: Area = Area(Point(20, -10), Point(30, -5));

pub fn part_one(_input: &str) -> Result<String, AocError> {
    // Because the vertical acceleration is constant, your speed when you return
    // to y=0 will always match your speed when you leave. Thus, the max speed
    // you can be going at that point is the distance to the bottom of the
    // target area, otherwise you'll overshoot in one step. So start at that
    // max speed and work down, first arc that goes through is the fastest.
    for i in 0..(INPUT.height()) {
        let vy = -INPUT.bottom() - i;
        for vx in 0..INPUT.right() {
            let guess = Arc { vx, vy, loc: Point(0, 0) };
            if guess.passes_through(&INPUT) {
                return Ok(guess.max_height().to_string())
            }
        }
    }

    Err(AocError::Misc("No solution".to_string()))
}

pub fn part_two(_input: &str) -> Result<String, AocError> {
    let mut count = 0;

    // This is obviously the slow way but I don't feel like doing math tonight
    for vy in INPUT.bottom()..=-INPUT.bottom() {
        for vx in 0..=INPUT.right() {
            let guess = Arc { vx, vy, loc: Point(0, 0) };
            if guess.passes_through(&INPUT) {
                count += 1;
            }
        }
    }

    Ok(count.to_string())
}
