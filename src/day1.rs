use std::error::Error;

pub fn parse_input(input: &str) -> Vec<i32> {
  input
    .lines()
    .map(|line| line.parse::<i32>().unwrap())
    .collect()
}

fn mass_converter(x: i32) -> i32 {
  (x / 3) - 2
}

fn recursive_mass_converter(x: i32) -> i32 {
  let fuel = mass_converter(x);
  if fuel < 1 {
    0
  } else {
    fuel + recursive_mass_converter(fuel)
  }
}

fn loop_mass_converter(x: i32) -> i32 {
  let mut total_fuel = mass_converter(x);

  let mut fuel = mass_converter(total_fuel);
  while fuel > 0 {
    total_fuel += fuel;
    fuel = mass_converter(fuel);
  }
  total_fuel
}

pub fn part_two(inputs: &[i32]) -> i32 {
  inputs
    .iter()
    .fold(0, |acc, item| acc + recursive_mass_converter(*item))
}

pub fn part_two_loop(inputs: &[i32]) -> i32 {
  inputs
    .iter()
    .fold(0, |acc, item| acc + loop_mass_converter(*item))
}

pub fn part_one(inputs: &[i32]) -> i32 {
  inputs
    .iter()
    .fold(0, |acc, item| acc + mass_converter(*item))
}
#[cfg(test)]
pub mod test {
  use super::{loop_mass_converter, mass_converter, recursive_mass_converter};

  #[test]
  fn day1_part_one_mass() {
    assert_eq!(mass_converter(12), 2);
    assert_eq!(mass_converter(14), 2);
    assert_eq!(mass_converter(1969), 654);
    assert_eq!(mass_converter(100756), 33583);
  }
  #[test]
  fn day1_part_two_mass() {
    assert_eq!(recursive_mass_converter(12), 2);
    assert_eq!(recursive_mass_converter(14), 2);
    assert_eq!(recursive_mass_converter(1969), 966);
    assert_eq!(recursive_mass_converter(100756), 50346);
  }

  #[test]
  fn day1_part_two_mass_loop() {
    assert_eq!(loop_mass_converter(12), 2);
    assert_eq!(loop_mass_converter(14), 2);
    assert_eq!(loop_mass_converter(1969), 966);
    assert_eq!(loop_mass_converter(100756), 50346);
  }
}
