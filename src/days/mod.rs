pub mod one;
pub mod two;
pub mod three;
pub mod four;
pub mod five;
pub mod six;
pub mod seven;
pub mod eight;
pub mod nine;
pub mod ten;
pub mod eleven;
pub mod twelve;
pub mod thirteen;
pub mod fourteen;
pub mod fifteen;
pub mod sixteen;
pub mod seventeen;
pub mod eighteen;
pub mod nineteen;
pub mod twenty;
pub mod twentyone;
pub mod twentytwo;
pub mod twentythree;
pub mod twentyfour;
pub mod twentyfive;

use crate::aoc_error::AocError;

type Solution = fn(&str) -> Result<String, AocError>;

pub struct Day {
    pub day: String,
    pub name: String,
    pub part_one: Solution,
    pub part_two: Solution
}

macro_rules! match_day {
    ( $target:ident $( $name:literal => $module:ident ),* ) => {
        match $target {
            $(
                $name => Some(Day {
                    day: $name.to_string(),
                    name: $module::NAME.to_string(),
                    part_one: $module::part_one,
                    part_two: $module::part_two
                }),
            )*
            _ => None
        }
    }
}

pub fn get_day(day: &str) -> Option<Day> {
    match_day! {
        day

        "1" => one,
        "2" => two,
        "3" => three,
        "4" => four,
        "5" => five,
        "6" => six,
        "7" => seven,
        "8" => eight,
        "9" => nine,
        "10" => ten,
        "11" => eleven,
        "12" => twelve,
        "13" => thirteen,
        "14" => fourteen,
        "15" => fifteen,
        "16" => sixteen,
        "17" => seventeen,
        "18" => eighteen,
        "19" => nineteen,
        "20" => twenty,
        "21" => twentyone,
        "22" => twentytwo,
        "23" => twentythree,
        "24" => twentyfour,
        "25" => twentyfive
    }
}
