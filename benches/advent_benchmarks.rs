use std::fs::File;
use std::io;
use std::io::prelude::*;

use criterion::{criterion_group, criterion_main, Criterion};
use lib::day1;

fn open_file(filename: &str) -> std::io::Result<String> {
  let mut file = File::open(filename)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  Ok(contents)
}

pub fn day1_p1_benchmark(c: &mut Criterion) {
  let day1_input = day1::parse_input(&open_file("./inputs/day1.txt").unwrap());
  c.bench_function("Part 1", |b| b.iter(|| day1::part_one(&day1_input)));
}

pub fn day1_p2_benchmark_recursive(c: &mut Criterion) {
  let day1_input: Vec<i32> = day1::parse_input(&open_file("./inputs/day1.txt").unwrap());
  c.bench_function("Part 2 Recursion", |b| {
    b.iter(|| day1::part_two(&day1_input))
  });
}

pub fn day1_p2_benchmark_loop(c: &mut Criterion) {
  let day1_input: Vec<i32> = day1::parse_input(&open_file("./inputs/day1.txt").unwrap());
  c.bench_function("Part 2 Loop", |b| {
    b.iter(|| day1::part_two_loop(&day1_input))
  });
}

criterion_group!(
  day1,
  day1_p1_benchmark,
  day1_p2_benchmark_loop,
  day1_p2_benchmark_recursive
);
criterion_main!(day1);
