use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;
use crate::aoc_error::AocError;

pub const NAME: &str = "Passage Pathing";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Room {
    Start,
    End,
    Big(usize),
    Small(usize)
}

impl<'a> Room {
    fn parse(s: &'a str, names: &mut HashMap<&'a str, usize>) -> Room {
        lazy_static::lazy_static! {
            static ref UPPERCASE: Regex = Regex::new("[A-Z]+").unwrap();
        }

        if s == "start" {
            Room::Start
        } else if s == "end" {
            Room::End
        } else if UPPERCASE.is_match(s) {
            if let Some(n) = names.get(s) {
                Room::Big(*n)
            } else {
                let n = 1 << names.len();
                names.insert(s, n);
                Room::Big(n)
            }
        } else {
            if let Some(n) = names.get(s) {
                Room::Small(*n)
            } else {
                let n = 1 << names.len();
                names.insert(s, n);
                Room::Small(n)
            }
        }
    }

    fn index(&self) -> usize {
        match self {
            Room::Start => 0,
            // We don't ever try to check end's neighbors, so this is just a
            // number larger than the number of bits in usize
            Room::End => 200,
            Room::Big(n) => n.trailing_zeros() as usize + 1,
            Room::Small(n) => n.trailing_zeros() as usize + 1
        }
    }
}

trait RoomPath: std::fmt::Debug {
    fn peek(&self) -> &Room;
    fn try_visit(&mut self, room: &Room) -> bool;
    fn unvisit(&mut self);
}

#[derive(Debug)]
struct RoomPathP1 {
    path: Vec<Room>,
    contains: usize
}

impl RoomPathP1 {
    fn new() -> Self {
        RoomPathP1 {
            path: vec![ Room::Start ],
            contains: 0
        }
    }
}

impl RoomPath for RoomPathP1 {
    fn peek(&self) -> &Room { &self.path[self.path.len() - 1] }

    fn try_visit(&mut self, room: &Room) -> bool {
        match room {
            Room::Start => false,
            Room::End => {
                self.path.push(room.clone());
                true
            },
            Room::Big(n) => {
                self.path.push(room.clone());
                self.contains |= n;
                true
            },
            Room::Small(n) => {
                if self.contains & n > 0 {
                    false
                } else {
                    self.path.push(room.clone());
                    self.contains |= n;
                    true
                }
            }
        }
    }

    fn unvisit(&mut self) {
        let last = self.path[self.path.len() - 1];
        match last {
            // This actually misrepresents whether we have Big rooms, since
            // it'll remove them from contains even if we've visited them more
            // than once previously. We don't really care about that though.
            Room::Big(n) => { self.contains &= !n; },
            Room::Small(n) => { self.contains &= !n; },
            _ => {}
        };

        self.path.pop();
    }
}

#[derive(Debug)]
struct RoomPathP2 {
    path: Vec<Room>,
    contains: usize,
    doubled: usize
}

impl RoomPathP2 {
    fn new() -> Self {
        RoomPathP2 {
            path: vec![ Room::Start ],
            contains: 0,
            doubled: 0
        }
    }
}

impl RoomPath for RoomPathP2 {
    fn peek(&self) -> &Room { &self.path[self.path.len() - 1] }

    fn try_visit(&mut self, room: &Room) -> bool {
        match room {
            Room::Start => false,
            Room::End => {
                self.path.push(room.clone());
                true
            }
            Room::Big(n) => {
                self.path.push(room.clone());
                self.contains |= n;
                true
            }
            Room::Small(n) => {
                if self.contains & n > 0 {
                    if self.doubled == 0 {
                        self.path.push(room.clone());
                        self.doubled = *n;
                        true
                    } else {
                        false
                    }
                } else {
                    self.path.push(room.clone());
                    self.contains |= n;
                    true
                }
            }
        }
    }

    fn unvisit(&mut self) {
        let last = self.path[self.path.len() - 1];
        match last {
            // This actually misrepresents whether we have Big rooms, since
            // it'll remove them from contains even if we've visited them more
            // than once previously. We don't really care about that though.
            Room::Big(n) => { self.contains &= !n; },
            Room::Small(n) => {
                if self.doubled == n {
                    self.doubled = 0;
                } else {
                    self.contains &= !n;
                }
            },
            _ => {}
        }

        self.path.pop();
    }
}

#[derive(Debug)]
struct RoomGraph {
    edges: Vec<Vec<Room>>
}

impl FromStr for RoomGraph {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = RoomGraph { edges: Vec::new() };
        let mut names = HashMap::new();

        for line in s.lines() {
            let (left, right) = line.split_once("-")
                .ok_or_else(|| AocError::Misc("Bad line".to_string()))?;
            let left = Room::parse(left, &mut names);
            let right = Room::parse(right, &mut names);

            graph.add_edge(left.clone(), right.clone());
            graph.add_edge(right, left);
        }

        Ok(graph)
    }
}

impl RoomGraph {
    fn add_edge(&mut self, from: Room, to: Room) {
        let index = from.index();

        while index >= self.edges.len() {
            self.edges.push(Vec::new());
        }

        self.edges[index].push(to);
    }

    fn count_paths_to_end<P: RoomPath>(&self, path: &mut P) -> usize {
        let end = path.peek();

        if end == &Room::End {
            1
        } else {
            self.edges[end.index()]
                .iter()
                .map(|neighbor| {
                    if path.try_visit(neighbor) {
                        let count = self.count_paths_to_end(path);
                        path.unvisit();
                        count
                    } else {
                        0
                    }
                })
                .sum()
        }
    }

    fn count_paths_p1(&self) -> usize {
        self.count_paths_to_end(&mut RoomPathP1::new())
    }

    fn count_paths_p2(&self) -> usize {
        self.count_paths_to_end(&mut RoomPathP2::new())
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let graph: RoomGraph = input.parse()?;
    let count = graph.count_paths_p1();

    Ok(count.to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let graph: RoomGraph = input.parse()?;
    let count = graph.count_paths_p2();

    Ok(count.to_string())
}