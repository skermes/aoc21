use std::io::Read;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc21::days::one;
use aoc21::days::two;
use aoc21::days::three;
use aoc21::days::four;
use aoc21::days::five;
use aoc21::days::six;
use aoc21::days::seven;
use aoc21::days::eight;
use aoc21::days::nine;
use aoc21::days::ten;
use aoc21::days::eleven;
use aoc21::days::twelve;
use aoc21::days::thirteen;
use aoc21::days::fourteen;
use aoc21::days::sixteen;
use aoc21::days::seventeen;
use aoc21::days::twentyone;

fn get_input(day: u8) -> String {
  let mut input_file = std::fs::File::open(format!("inputs/{}.txt", day)).unwrap();
  let mut buffer = String::new();
  input_file.read_to_string(&mut buffer).unwrap();
  buffer.trim().to_string()
}

pub fn day_one(c: &mut Criterion) {
  let input = get_input(1);
  c.bench_function("Day One Part One", |b| b.iter(|| one::part_one(black_box(&input))));
  c.bench_function("Day One Part Two", |b| b.iter(|| one::part_two(black_box(&input))));
}

pub fn day_two(c: &mut Criterion) {
  let input = get_input(2);
  c.bench_function("Day Two Part One", |b| b.iter(|| two::part_one(black_box(&input))));
  c.bench_function("Day Two Part Two", |b| b.iter(|| two::part_two(black_box(&input))));
}

pub fn day_three(c: &mut Criterion) {
  let input = get_input(3);
  c.bench_function("Day Three Part One", |b| b.iter(|| three::part_one(black_box(&input))));
  c.bench_function("Day Three Part Two", |b| b.iter(|| three::part_two(black_box(&input))));
}

pub fn day_four(c: &mut Criterion) {
  let input = get_input(4);
  c.bench_function("Day Four Part One", |b| b.iter(|| four::part_one(black_box(&input))));
  c.bench_function("Day Four Part Two", |b| b.iter(|| four::part_two(black_box(&input))));
}

pub fn day_five(c: &mut Criterion) {
  let input = get_input(5);
  c.bench_function("Day Five Part One", |b| b.iter(|| five::part_one(black_box(&input))));
  c.bench_function("Day Five Part Two", |b| b.iter(|| five::part_two(black_box(&input))));
}

pub fn day_six(c: &mut Criterion) {
  let input = get_input(6);
  c.bench_function("Day Six Part One", |b| b.iter(|| six::part_one(black_box(&input))));
  c.bench_function("Day Six Part Two", |b| b.iter(|| six::part_two(black_box(&input))));
}

pub fn day_seven(c: &mut Criterion) {
  let input = get_input(7);
  c.bench_function("Day Seven Part One", |b| b.iter(|| seven::part_one(black_box(&input))));
  c.bench_function("Day Seven Part Two", |b| b.iter(|| seven::part_two(black_box(&input))));
}

pub fn day_eight(c: &mut Criterion) {
  let input = get_input(8);
  c.bench_function("Day Eight Part One", |b| b.iter(|| eight::part_one(black_box(&input))));
  c.bench_function("Day Eight Part Two", |b| b.iter(|| eight::part_two(black_box(&input))));
}

pub fn day_nine(c: &mut Criterion) {
  let input = get_input(9);
  c.bench_function("Day Nine Part One", |b| b.iter(|| nine::part_one(black_box(&input))));
  c.bench_function("Day Nine Part Two", |b| b.iter(|| nine::part_two(black_box(&input))));
}

pub fn day_ten(c: &mut Criterion) {
  let input = get_input(10);
  c.bench_function("Day Ten Part One", |b| b.iter(|| ten::part_one(black_box(&input))));
  c.bench_function("Day Ten Part Two", |b| b.iter(|| ten::part_two(black_box(&input))));
}

pub fn day_eleven(c: &mut Criterion) {
  let input = get_input(11);
  c.bench_function("Day Eleven Part One", |b| b.iter(|| eleven::part_one(black_box(&input))));
  c.bench_function("Day Eleven Part Two", |b| b.iter(|| eleven::part_two(black_box(&input))));
}

pub fn day_twelve(c: &mut Criterion) {
  let input = get_input(12);
  c.bench_function("Day Twelve Part One", |b| b.iter(|| twelve::part_one(black_box(&input))));
  c.bench_function("Day Twelve Part Two", |b| b.iter(|| twelve::part_two(black_box(&input))));
}

pub fn day_thirteen(c: &mut Criterion) {
  let input = get_input(13);
  c.bench_function("Day Thirteen Part One", |b| b.iter(|| thirteen::part_one(black_box(&input))));
  c.bench_function("Day Thirteen Part Two", |b| b.iter(|| thirteen::part_two(black_box(&input))));
}

pub fn day_fourteen(c: &mut Criterion) {
  let input = get_input(14);
  c.bench_function("Day Fourteen Part One", |b| b.iter(|| fourteen::part_one(black_box(&input))));
  c.bench_function("Day Fourteen Part Two", |b| b.iter(|| fourteen::part_two(black_box(&input))));
}

pub fn day_sixteen(c: &mut Criterion) {
  let input = get_input(16);
  c.bench_function("Day Sixteen Part One", |b| b.iter(|| sixteen::part_one(black_box(&input))));
  c.bench_function("Day Sixteen Part Two", |b| b.iter(|| sixteen::part_two(black_box(&input))));
}

pub fn day_seventeen(c: &mut Criterion) {
  let input = get_input(17);
  c.bench_function("Day Seventeen Part One", |b| b.iter(|| seventeen::part_one(black_box(&input))));
  c.bench_function("Day Seventeen Part Two", |b| b.iter(|| seventeen::part_two(black_box(&input))));
}

pub fn day_twentyone(c: &mut Criterion) {
  let input = get_input(21);
  c.bench_function("Day Twenty One Part One", |b| b.iter(|| twentyone::part_one(black_box(&input))));
  c.bench_function("Day Twenty One Part Two", |b| b.iter(|| twentyone::part_two(black_box(&input))));
}

criterion_group!(benches,
  day_one, day_two, day_three, day_four, day_five, day_six, day_seven,
  day_eight, day_nine, day_ten, day_eleven, day_twelve, day_thirteen,
  day_fourteen, day_sixteen, day_seventeen, day_twentyone);
criterion_main!(benches);
