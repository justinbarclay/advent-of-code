use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
fn open_file(filename: &str) -> std::io::Result<String> {
  let mut file = File::open(filename)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  Ok(contents)
}
mod day1;
mod day2;
mod day3;

fn main() {
  println!("Day 1");
  {
    let day1_input = day1::parse_input(&open_file("./inputs/day1.txt").unwrap());
    println!("Part 1: {}", day1::part_one(&day1_input));
    println!("Part 2: {}\n", day1::part_two(&day1_input));
  }

  println!("Day 2");
  let day2_input = day2::parse_input(&open_file("./inputs/day2.txt").unwrap());
  {
    println!("Part 1: {}", day2::part_one(&day2_input)[0]);
  }
  {
    let result = day2::part_two(&day2_input, 19690720);
    println!("Part 2: {}\n", 100 * result.0 + result.1);
  }

  println!("Day 3");
  let day3_input = day3::parse_input(&open_file("./inputs/day3.txt").unwrap());
  {
    println!("Part 1: {}", day3::part_one(&day3_input.0, &day3_input.1))
  }
  {
    println!("Part 2: {}\n", day3::part_two(&day3_input.0, &day3_input.1))
  }
}
