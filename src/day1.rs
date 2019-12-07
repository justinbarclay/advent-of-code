pub const INPUT: [i32; 100] = [
  99603, 121503, 86996, 72052, 112039, 106616, 123581, 123171, 52480, 68686, 66395, 102661, 110250,
  73289, 105725, 123802, 75488, 79426, 98634, 76095, 50852, 141405, 112388, 72180, 103300, 124602,
  104531, 94751, 63270, 139027, 145939, 62275, 91812, 74751, 144010, 60221, 62821, 51080, 149802,
  53067, 102574, 131339, 78942, 88430, 105314, 72764, 55214, 79095, 97458, 68699, 106974, 141492,
  57673, 141866, 139355, 134222, 52145, 83293, 144322, 70741, 107873, 123638, 141011, 133249,
  99065, 120480, 100767, 136550, 147323, 146988, 65583, 141287, 53097, 50662, 121124, 94886, 59344,
  93981, 112492, 149136, 56647, 96430, 63968, 117987, 138475, 125958, 74967, 64480, 104644, 70273,
  50671, 147116, 147101, 89096, 94697, 83282, 74533, 68418, 145578, 59032,
];
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
