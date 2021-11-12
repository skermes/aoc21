#[macro_use]
extern crate lazy_static;

mod aoc_error;
mod days;

use std::io::Read;
use std::env;
use std::fmt::Display;
use std::time::{Instant, Duration};

use reqwest;
use chrono::prelude::{Utc, TimeZone};

use crate::aoc_error::AocError;
use crate::days::{get_day, Day};

fn format_result<V, E>(result: &Result<V, E>) -> String
    where V: Display,
          E: Display
{
    match result {
        Err(error) => format!("{}", error),
        Ok(solution) => format!("{}", solution)
    }
}

fn format_duration(duration: Duration) -> String {
    let micros = duration.as_micros();

    if micros < 1_000 {
        format!("{}\u{03BC}s", micros)
    } else if micros < 1_000_000 {
        format!("{:.1}ms", (micros as f64) / 1_000.)
    } else {
        format!("{:.1}s", (micros as f64) / 1_000_000.)
    }
}

struct DayReport {
    file_duration: Duration,
    part_one_result: Result<String, AocError>,
    part_one_duration: Duration,
    part_two_result: Result<String, AocError>,
    part_two_duration: Duration
}

fn get_input(day: &str) -> Result<String, AocError> {
    let fname = format!("inputs/{}.txt", day);
    let path = std::path::Path::new(&fname);

    if !path.exists() {
        let now = Utc::now();
        // AoC is always run so that puzzles unlock at midnight EST, UTC-5.
        // Therefore, if it's before 5 AM UTC on Dec N 2021, the puzzle is
        // unavailable, and there's no point downloading it.
        if now < Utc.ymd(2021, 12, day.parse()?).and_hms(5, 0, 0) {
            return Err(AocError::TooEarly);
        }

        let mut session_cookie = String::new();
        let mut cookie_file = std::fs::File::open(".advent-session-cookie")?;
        cookie_file.read_to_string(&mut session_cookie)?;

        let client = reqwest::blocking::Client::new();
        let res = client
            .get(format!("https://adventofcode.com/2021/day/{}/input", day))
            .header("Cookie", format!("session={}", session_cookie))
            .send()?;

        let input = res.text()?;
        if input.starts_with("Please don't repeatedly request this endpoint before it unlocks") {
            // Whoops, my date math was wrong.
            return Err(AocError::TooEarly);
        }

        std::fs::write(path, input)?;
    }

    let mut input_file = std::fs::File::open(path)?;
    let mut buffer = String::new();
    input_file.read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn run(day: &Day) -> Result<DayReport, AocError> {
    let start = Instant::now();
    let input = get_input(&day.day)?;
    let file_duration = start.elapsed();

    let start = Instant::now();
    let result_one = (day.part_one)(&input);
    let duration_one = start.elapsed();

    let start = Instant::now();
    let result_two = (day.part_two)(&input);
    let duration_two = start.elapsed();

    Ok(DayReport {
        file_duration,
        part_one_result: result_one,
        part_one_duration: duration_one,
        part_two_result: result_two,
        part_two_duration: duration_two
    })
}

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().collect();

    let mut days: Vec<Day> = Vec::new();
    if args.len() < 2 {
        for i in 1..26 {
            if let Some(day) = get_day(&i.to_string()) {
                days.push(day);
            }
        }
    } else if let Some(day) = get_day(&args[1]) {
        days.push(day);
    } else {
        println!("No implementation for day {}.", &args[1]);
        return;
    }

    let mut total_problem_duration = Duration::new(0, 0);
    let mut total_file_duration = Duration::new(0, 0);

    for day in days {
        let report = run(&day);

        println!("\nDay {}: {}", day.day, day.name);
        match report {
            Err(error) => println!("  {}", error),
            Ok(report) => {
                println!(
                    "  Part One: {:40} {:>7}",
                    format_result(&report.part_one_result),
                    format_duration(report.part_one_duration)
                );
                println!(
                    "  Part Two: {:40} {:>7}",
                    format_result(&report.part_two_result),
                    format_duration(report.part_two_duration)
                );
                println!(
                    "  Input:    {:40} {:>7}",
                    "",
                    format_duration(report.file_duration)
                );
                total_file_duration += report.file_duration;
                total_problem_duration += report.part_one_duration + report.part_two_duration;
            }
        }
    }

    let total_duration = start.elapsed();
    let overhead = total_duration - total_problem_duration - total_file_duration;
    println!("{:─<60}", "");
    println!("Time - total:    {:>43}", format_duration(total_duration));
    println!("       problem:  {:>43}", format_duration(total_problem_duration));
    println!("       input:    {:>43}", format_duration(total_file_duration));
    println!("       overhead: {:>43}", format_duration(overhead));
}
