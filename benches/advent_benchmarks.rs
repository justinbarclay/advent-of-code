use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lib::day1;

pub fn day1_p1_benchmark(c: &mut Criterion) {
  c.bench_function("Part 1", |b| b.iter(|| day1::part_one(&day1::INPUT)));
}
pub fn day1_p2_benchmark(c: &mut Criterion) {
  c.bench_function("Part 2 Recursion", |b| {
    b.iter(|| day1::part_two(&day1::INPUT))
  });
}
pub fn day1_p2_loop_benchmark(c: &mut Criterion) {
  c.bench_function("Part 2 Loop", |b| {
    b.iter(|| day1::part_two_loop(&day1::INPUT))
  });
}

criterion_group!(
  day1,
  day1_p1_benchmark,
  day1_p2_benchmark,
  day1_p2_loop_benchmark
);
criterion_main!(day1);
