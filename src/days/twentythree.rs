#![allow(dead_code)]

use crate::aoc_error::AocError;
use lazy_static::lazy_static;

pub const NAME: &str = "Amphipod";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Species { Amber, Bronze, Copper, Desert }

impl Species {
    fn cost(&self) -> usize {
        match self {
            Species::Amber => 1,
            Species::Bronze => 10,
            Species::Copper => 100,
            Species::Desert => 1000
        }
    }

    // Instead of indexing the rooms separately from the grid as a whole,
    // These are the x-positions of the rooms entrance in the hallway.
    fn desired_room(&self) -> usize {
        match self {
            Species::Amber => 2,
            Species::Bronze => 4,
            Species::Copper => 6,
            Species::Desert => 8
        }
    }

    fn char(&self) -> char {
        match self {
            Species::Amber => 'A',
            Species::Bronze => 'B',
            Species::Copper => 'C',
            Species::Desert => 'D'
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Amphipod {
    Initial {
        species: Species,
        room: usize,
        depth: usize
    },
    Hallway {
        species: Species,
        position: usize
    },
    Final {
        species: Species,
        room: usize,
        depth: usize
    }
}

impl Amphipod {
    fn is_in_room(&self, target: usize) -> bool {
        match self {
            Amphipod::Initial { room, .. } => room == &target,
            Amphipod::Final { room, .. } => room == &target,
            Amphipod::Hallway { .. } => false
        }
    }

    fn hallway_position(&self) -> usize {
        match self {
            // Obviously incorrect, but this is just the last spot after the
            // end of the hallway.
            Amphipod::Initial { .. } => 11,
            Amphipod::Final { .. } => 11,
            Amphipod::Hallway { position, .. } => *position
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Burrow {
    cost: usize,
    hallway_length: usize,
    amphipods: Vec<Amphipod>
}

impl Burrow {
    fn sorted(&self) -> bool {
        self.amphipods
            .iter()
            .all(|amphipod| {
                match amphipod {
                    Amphipod::Hallway { .. } => false,
                    Amphipod::Initial { species, room, .. } =>
                        room == &species.desired_room(),
                    Amphipod::Final { species, room, .. } =>
                        room == &species.desired_room()
                }
            })
    }

    fn next_states_moving_pod(&self, amphipod: Amphipod) -> Vec<Burrow> {
        match amphipod {
            Amphipod::Final { .. } => Vec::new(),
            Amphipod::Hallway { species, position } => {
                let target = species.desired_room();
                let pods_in_room = self.amphipods
                    .iter()
                    .filter(|pod| pod.is_in_room(target))
                    .count();

                // TODO: Don't move if other pods aren't right species

                if pods_in_room >= 2 {
                    Vec::new()
                } else {
                    let mut burrow = self.clone();
                    let depth = 1 - pods_in_room;
                    let cost = if position > target {
                        position - target + depth + 1
                    } else {
                        target - position + depth + 1
                    };
                    let index = burrow.amphipods
                        .iter()
                        .position(|pod| pod == &amphipod)
                        .unwrap();
                    burrow.amphipods.remove(index);
                    burrow.amphipods.push(Amphipod::Final {
                        species,
                        depth,
                        room: target,
                    });
                    burrow.cost += cost;
                    vec![ burrow ]
                }
            },
            Amphipod::Initial { species, room, depth } => {
                if depth == 1 {
                    let pods_in_room = self.amphipods
                        .iter()
                        .filter(|pod| pod.is_in_room(room))
                        .count();
                    // Another pod is blocking us, can't get out
                    if pods_in_room == 2 {
                        return Vec::new()
                    }
                }

                let mut burrows = Vec::new();

                let left_hallway_spot = self.amphipods
                    .iter()
                    .map(|pod| pod.hallway_position())
                    .filter(|pod| pod < &room)
                    .max()
                    .map_or(0, |p| p + 1);
                let right_hallway_spot = self.amphipods
                    .iter()
                    .map(|pod| pod.hallway_position())
                    .filter(|pod| pod > &room)
                    .min()
                    .map_or(self.hallway_length - 1, |p| p - 1);

                for position in left_hallway_spot..=right_hallway_spot {
                    // Skip
                    if position == 2 || position == 4 || position == 6 || position == 8 {
                        continue;
                    }

                    let cost = if position > room {
                        position - room + depth + 1
                    } else {
                        room - position + depth + 1
                    };
                    let mut burrow = self.clone();
                    let index = burrow.amphipods
                        .iter()
                        .position(|pod| pod == &amphipod)
                        .unwrap();
                    burrow.amphipods.remove(index);
                    burrow.amphipods.push(Amphipod::Hallway {
                        species, position
                    });
                    burrow.cost += cost;
                    burrows.push(burrow);
                }

                burrows
            }
        }
    }

    fn next_states(&self) -> Vec<Burrow> {
        todo!()
    }
}

lazy_static! {
    static ref EXAMPLE_BURROW: Burrow = Burrow {
        cost: 0,
        hallway_length: 11,
        amphipods: vec![
            Amphipod::Initial {
                species: Species::Bronze,
                room: 2,
                depth: 0
            },
            Amphipod::Initial {
                species: Species::Amber,
                room: 2,
                depth: 1
            },
            Amphipod::Initial {
                species: Species::Copper,
                room: 4,
                depth: 0
            },
            Amphipod::Initial {
                species: Species::Desert,
                room: 4,
                depth: 1
            },
            Amphipod::Initial {
                species: Species::Bronze,
                room: 6,
                depth: 0
            },
            Amphipod::Initial {
                species: Species::Copper,
                room: 6,
                depth: 1
            },
            Amphipod::Initial {
                species: Species::Desert,
                room: 8,
                depth: 0
            },
            Amphipod::Initial {
                species: Species::Amber,
                room: 8,
                depth: 1
            }
        ]
    };
}

pub fn part_one(_input: &str) -> Result<String, AocError> {
    Ok("Not implemented".to_string())
}

pub fn part_two(_input: &str) -> Result<String, AocError> {
    Ok("Not implemented".to_string())
}
