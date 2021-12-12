use std::collections::VecDeque;
use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Smoke Basin";

#[derive(Debug)]
struct LavaTubeArea {
    locations: Vec<u32>,
    width: usize
}

impl FromStr for LavaTubeArea {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let first_line = lines.next().ok_or_else(|| AocError::Misc("Empty input".to_string()))?;
        let width = first_line.len();

        let mut locations = first_line.chars()
            .map(|c| c.to_digit(10).ok_or_else(|| AocError::Misc(format!("Bad digit {}", c))))
            .collect::<Result<Vec<u32>, AocError>>()?;

        locations.extend(lines
            .flat_map(|l| l.chars()
                .map(|c| c.to_digit(10).ok_or_else(|| AocError::Misc(format!("Bad digit {}", c))))
            )
            .collect::<Result<Vec<u32>, AocError>>()?
        );

        Ok(LavaTubeArea { locations, width })
    }
}

impl LavaTubeArea {
    fn neighbors(&self, position: usize) -> Vec<usize> {
        let mut positions = Vec::with_capacity(4);

        // Mod 0 is left edge
        if position % self.width != 0 { positions.push(position - 1); }
        // right edge
        if position % self.width != (self.width - 1) { positions.push(position + 1); }
        // top row
        if position >= self.width { positions.push(position - self.width); }
        // bottom row
        if position <= (self.locations.len() - self.width) { positions.push(position + self.width); }

        positions
    }

    fn is_low_point(&self, position: usize) -> bool {
        let height = self.locations[position];
        self.neighbors(position)
            .iter()
            .all(|p| height < self.locations[*p])
    }

    fn low_point_risk(&self) -> u32 {
        (0..self.locations.len())
            .filter(|p| self.is_low_point(*p))
            .map(|p| self.locations[p] + 1)
            .sum()
    }

    // Destructively count the locations in a basin by interatively setting each
    // location to 9 so that we know not to count it again.
    fn flood_fill_basin(&mut self, low_point: usize) -> usize {
        let mut to_visit = VecDeque::from([ low_point ]);
        let mut size = 0;

        while !to_visit.is_empty() {
            // Safe unwrap b/c of len check above.
            let next = to_visit.pop_front().unwrap();
            if self.locations[next] == 9 { continue; }

            let neighbors = self.neighbors(next);
            for neighbor in neighbors {
                if self.locations[neighbor] != 9 {
                    to_visit.push_back(neighbor);
                }
            }

            self.locations[next] = 9;
            size += 1;
        }

        size
    }

    fn basins(&mut self) -> Vec<usize> {
        let mut sizes = Vec::new();

        for loc in 0..self.locations.len() {
            if self.is_low_point(loc) {
                sizes.push(self.flood_fill_basin(loc));
            }
        }

        sizes
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let area: LavaTubeArea = input.parse()?;

    Ok(area.low_point_risk().to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let mut area: LavaTubeArea = input.parse()?;
    let mut basins = area.basins();

    if basins.len() < 3 {
        return Err(AocError::Misc("Too few lava tube basins".to_string()));
    }

    basins.sort();
    let area_product = basins[basins.len() - 1] *
                       basins[basins.len() - 2] *
                       basins[basins.len() - 3];

    Ok(area_product.to_string())
}
