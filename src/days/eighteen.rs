use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use crate::aoc_error::AocError;

pub const NAME: &str = "Snailfish";

// #[derive(Debug)]
// struct Element {
//     number: u32,
//     depth: u8
// }

// #[derive(Debug)]
// struct Number {
//     elements: Vec<Element>
// }

// impl FromStr for Number {
//     type Err = AocError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut depth = 0;
//         let mut elements = Vec::new();

//         for c in s.chars() {
//             match c {
//                 '[' => depth += 1,
//                 ']' => depth -= 1,
//                 '0'..='9' => elements.push(Element{
//                     number: c.to_digit(10)
//                         .ok_or_else(|| AocError::Misc("Bad digit".to_string()))?,
//                     depth: depth
//                 }),
//                 _ => {}
//             };
//         }

//         Ok(Number { elements })
//     }
// }

// impl Number {
//     fn explode(&mut self) -> bool {
//         for (i, elem) in self.elements.iter().enumerate() {
//             if elem.depth >= 5 {
//                 if i > 0 {
//                     self.elements[i-1].number += elem.number;
//                 }

//                 if i < self.elements.len() - 2 {
//                     self.elements[i+2].number += self.elements[i+1].number;
//                 }

//                 self.elements.remove(i + 1);
//                 self.elements[i].number = 0;
//                 self.elements[i].depth -= 1;

//                 return true;
//             }
//         }

//         false
//     }

//     fn split(&mut self) -> bool {
//         for (i, elem) in self.elements.iter().enumerate() {
//             if elem.number >= 10 {
//                 let adjustment = elem.number % 2;
//                 let value = elem.number / 2;
//                 let depth = elem.depth;

//                 self.elements.insert(i + 1, Element {
//                     number: value + adjustment,
//                     depth: depth
//                 });
//                 self.elements[i].number = value;
//                 self.elements[i].depth += 1;

//                 return true;
//             }
//         }

//         false
//     }

//     fn add(&mut self, other: Number) {
//         self.elements.extend(other.elements);
//         for elem in self.elements.iter_mut() {
//             elem.depth += 1;
//         }
//     }

//     fn reduce(&mut self) {
//         loop {
//             if self.explode() { continue; }
//             if self.split() { continue; }

//             return;
//         }
//     }
// }

#[derive(Debug, Clone)]
enum Number {
    Literal(u8),
    Pair(Box<Number>, Box<Number>)
}

impl FromStr for Number {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
            let mut depth = 0;
            for i in 0..s.len() {
                let c = &s[i..i+1];
                if c == "[" {
                    depth += 1;
                } else if c == "]" {
                    depth -= 1;
                } else if c == "," && depth == 1 {
                    let left = s[1..i].parse()?;
                    let right = s[i+1..s.len()-1].parse()?;
                    return Ok(Number::Pair(Box::new(left), Box::new(right)))
                }
            }
            Err(AocError::Misc("No middle comma in pair".to_string()))
        } else {
            Ok(Number::Literal(
                s.parse()?
            ))
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Number::Literal(n) => write!(f, "{}", n),
            Number::Pair(left, right) => write!(f, "[{},{}]", left, right)
        }
    }
}

impl Number {
    // Returns a path to the first exploding element. Path is a vec of 0
    // for left and 1 for right.
    fn find_exploder(&self, depth: u8) -> Option<Vec<u8>> {
        match self {
            Number::Literal(_) => None,
            Number::Pair(left, right) => {
                if depth == 4 {
                    Some(Vec::new())
                } else if let Some(mut path) = left.find_exploder(depth + 1) {
                    path.insert(0, 1);
                    Some(path)
                } else if let Some(mut path) = right.find_exploder(depth + 1) {
                    path.insert(0, 2);
                    Some(path)
                } else {
                    None
                }
            }
        }
    }

    fn find_at_mut(&mut self, path: &[u8]) -> Option<&mut Number> {
        if path.is_empty() {
            Some(self)
        } else {
            match self {
                Number::Literal(_) => Some(self),
                Number::Pair(left, right) => {
                    if path[0] == 1 {
                        left.find_at_mut(&path[1..])
                    } else if path[0] == 2 {
                        right.find_at_mut(&path[1..])
                    } else {
                        None
                    }
                }
            }
        }
    }

    fn find_left_mut(&mut self, path: &[u8]) -> Option<&mut Number> {
        if path.iter().all(|d| *d == 1) {
            return None;
        }

        let mut left_path = path.to_vec();
        for dir in left_path.iter_mut().rev() {
            if *dir == 2 {
                *dir = 1;
                break;
            } else {
                *dir = 2;
            }
        }

        self.find_at_mut(&left_path)
    }

    fn find_right_mut(&mut self, path: &[u8]) -> Option<&mut Number> {
        if path.iter().all(|d| *d == 2) {
            return None;
        }

        let mut right_path = path.to_vec();
        for dir in right_path.iter_mut().rev() {
            if *dir == 1 {
                *dir = 2;
                break;
            } else {
                *dir = 1;
            }
        }

        self.find_at_mut(&right_path)
    }

    #[allow(dead_code)]
    fn explode(&mut self) -> bool {
        if let Some(path) = self.find_exploder(0) {
            let exploder = self.find_at_mut(&path).unwrap();

            // let (lval, rval) = if let Number::Pair(left, right) = exploder {
            //     match (left, right) {
            //         (Number::Literal(lval), Number::Literal(rval)) => (lval, rval),
            //         _ => panic!("ph no")
            //     }
            // } else {
            //     panic!("oh no")
            // };

            // if let Some(left) = self.find_left_mut(&path) {
            //     if let Number::Literal(value) = left {
            //         *value += lval;
            //     } else {
            //         panic!("uh oh");
            //     }
            // }

            // if let Some(right) = self.find_right_mut(&path) {
            //     if let Number::Literal(value) == right {
            //         *value += rval;
            //     } else {
            //         panic("oh no");
            //     }
            // }

            *exploder = Number::Literal(0);
            true
        } else {
            false
        }
    }
}

pub fn part_one(_input: &str) -> Result<String, AocError> {
//     let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
// [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
// [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
// [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
// [7,[5,[[3,8],[1,4]]]]
// [[2,[2,2]],[8,[8,1]]]
// [2,9]
// [1,[[[9,3],9],[[9,0],[0,7]]]]
// [[[5,[7,4]],7],1]
// [[[[4,2],2],6],[8,7]]";

//     let mut lines = input.lines();
//     let mut base: Number = lines.next().unwrap().parse()?;
//     for line in lines {
//         base.add(line.parse()?);
//         base.reduce();
//         println!("{:?}\n", base);
//     }

//     println!("{:?}", base);

    let tests = [
        "[[[[[9,8],1],2],3],4]",
        "[7,[6,[5,[4,[3,2]]]]]",
        "[[6,[5,[4,[3,2]]]],1]",
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
    ];
    for line in tests {
        let mut num: Number = line.parse()?;
        if let Some(path) = num.find_exploder(0) {
            println!("{}", num);
            println!("explode path  {:?}", path);
            println!("exploder      {:?}", num.find_at_mut(&path));
            println!("left explode  {:?}", num.find_left_mut(&path));
            println!("right explode {:?}\n", num.find_right_mut(&path));
        }
    }

    Ok("Not implemented".to_string())
}

pub fn part_two(_input: &str) -> Result<String, AocError> {
    Ok("Not implemented".to_string())
}
