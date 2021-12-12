use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;
use crate::aoc_error::AocError;

pub const NAME: &str = "Passage Pathing";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Room {
    Start,
    End,
    Big(String),
    Small(String)
}

impl FromStr for Room {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref UPPERCASE: Regex = Regex::new("[A-Z]+").unwrap();
        }

        let s = s.trim();

        if s == "start" {
            Ok(Room::Start)
        } else if s == "end" {
            Ok(Room::End)
        } else if UPPERCASE.is_match(s) {
            Ok(Room::Big(s.to_string()))
        } else {
            Ok(Room::Small(s.to_string()))
        }
    }
}

#[derive(Debug)]
struct RoomGraph {
    edges: HashMap<Room, Vec<Room>>
}

impl FromStr for RoomGraph {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = RoomGraph { edges: HashMap::new() };

        for line in s.lines() {
            let (left, right) = line.split_once("-")
                .ok_or_else(|| AocError::Misc("Bad line".to_string()))?;
            let left: Room = left.parse()?;
            let right: Room = right.parse()?;

            graph.add_edge(left.clone(), right.clone());
            graph.add_edge(right, left);
        }

        Ok(graph)
    }
}

impl RoomGraph {
    fn add_edge(&mut self, from: Room, to: Room) {
        if let Some(rooms) = self.edges.get_mut(&from) {
            rooms.push(to);
        } else {
            self.edges.insert(from, vec![to]);
        }
    }

    fn count_paths_to_end<P: RoomPath>(&self, path: P) -> Result<usize, AocError> {
        let end = path.peek();

        if end == &Room::End {
            Ok(1)
        } else {
            self.edges.get(end)
                .ok_or_else(|| AocError::Misc("Missing room".to_string()))?
                .iter()
                .map(|neighbor| {
                    if let Some(new_path) = path.try_visit(neighbor) {
                        self.count_paths_to_end(new_path)
                    } else {
                        Ok(0)
                    }
                })
                .fold(Ok(0), |acc, x| match (acc, x) {
                    (Ok(acc), Ok(x)) => Ok(acc + x),
                    (Err(e), _) => Err(e),
                    (_, Err(e)) => Err(e)
                })
        }
    }

    fn count_paths_p1(&self) -> Result<usize, AocError> {
        self.count_paths_to_end(RoomPathP1::new())
    }

    fn count_paths_p2(&self) -> Result<usize, AocError> {
        self.count_paths_to_end(RoomPathP2::new())
    }
}

trait RoomPath {
    fn peek(&self) -> &Room;
    fn try_visit(&self, room: &Room) -> Option<Self> where Self: Sized;
}

#[derive(Debug)]
struct RoomPathP1 {
    path: Vec<Room>
}

impl RoomPathP1 {
    fn new() -> Self { RoomPathP1 { path: vec![ Room::Start ] } }

    fn visit(&self, room: &Room) -> Self {
        let mut new_path = self.path.clone();
        new_path.push(room.clone());
        RoomPathP1 { path: new_path }
    }
}

impl RoomPath for RoomPathP1 {
    fn peek(&self) -> &Room { &self.path[self.path.len() - 1] }

    fn try_visit(&self, room: &Room) -> Option<Self> {
        match room {
            Room::Start => None,
            Room::End => Some(self.visit(room)),
            Room::Big(_) => Some(self.visit(room)),
            Room::Small(_) => {
                if self.path.contains(&room) {
                    None
                } else {
                    Some(self.visit(room))
                }
            }
        }
    }
}

#[derive(Debug)]
struct RoomPathP2 {
    path: Vec<Room>,
    doubled: bool
}

impl RoomPathP2 {
    fn new() -> Self {
        RoomPathP2 {
            path: vec![ Room::Start ],
            doubled: false
        }
    }

    fn visit(&self, room: &Room, is_double: bool) -> Self {
        let mut new_path = self.path.clone();
        new_path.push(room.clone());

        if is_double {
            RoomPathP2 {
                path: new_path,
                doubled: true
            }
        } else {
            RoomPathP2 {
                path: new_path,
                doubled: self.doubled
            }
        }
    }
}

impl RoomPath for RoomPathP2 {
    fn peek(&self) -> &Room { &self.path[self.path.len() - 1] }

    fn try_visit(&self, room: &Room) -> Option<Self> {
        match room {
            Room::Start => None,
            Room::End => Some(self.visit(room, false)),
            Room::Big(_) => Some(self.visit(room, false)),
            Room::Small(_) => {
                match (self.path.contains(&room), self.doubled) {
                    (false, _) => Some(self.visit(room, false)),
                    (true, false) => Some(self.visit(room, true)),
                    (true, true) => None
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let graph: RoomGraph = input.parse()?;
    let count = graph.count_paths_p1()?;

    Ok(count.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let graph: RoomGraph = input.parse()?;
    let count = graph.count_paths_p2()?;

    Ok(count.to_string())
}