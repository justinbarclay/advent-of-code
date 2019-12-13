pub fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
  let mut parsed_input: Vec<Vec<String>> = input
    .trim()
    .lines()
    .map(|line| {
      line
        .trim()
        .split(',')
        .map(|item| item.to_string())
        .collect()
    })
    .collect();

  (parsed_input[0].clone(), parsed_input[1].clone())
}

trait Substring {
  fn substring(&self, start: usize, end: usize) -> String;
}

impl Substring for String {
  fn substring(&self, start: usize, end: usize) -> String {
    let string = self.clone();
    string[string.char_indices().nth(start).unwrap().0..end].to_string()
  }
}

#[derive(Clone, Debug, PartialEq)]
struct Coordinate {
  x: i32,
  y: i32,
}

trait ManhattanDistance {
  fn manhattan_distance(&self, coordinate: Coordinate) -> i32;
}

impl ManhattanDistance for Coordinate {
  fn manhattan_distance(&self, coordinate: Coordinate) -> i32 {
    (self.x - coordinate.x).abs() + (self.y - coordinate.y).abs()
  }
}

fn movements_to_paths(path: &[String]) -> Vec<(Coordinate, Coordinate)> {
  let mut old_coordinates = Coordinate { x: 0, y: 0 };
  path
    .iter()
    .map(|step| match step.chars().next().unwrap() {
      'R' => {
        let new_coordinates = Coordinate {
          x: old_coordinates.x + step.substring(1, step.len()).parse::<i32>().unwrap(),
          y: old_coordinates.y,
        };
        let path = (old_coordinates.clone(), new_coordinates.clone());
        old_coordinates = new_coordinates;
        path
      }
      'L' => {
        let new_coordinates = Coordinate {
          x: old_coordinates.x - step.substring(1, step.len()).parse::<i32>().unwrap(),
          y: old_coordinates.y,
        };
        let path = (old_coordinates.clone(), new_coordinates.clone());
        old_coordinates = new_coordinates;
        path
      }
      'U' => {
        let new_coordinates = Coordinate {
          x: old_coordinates.x,
          y: old_coordinates.y - step.substring(1, step.len()).parse::<i32>().unwrap(),
        };
        let path = (old_coordinates.clone(), new_coordinates.clone());
        old_coordinates = new_coordinates;
        path
      }
      'D' => {
        let new_coordinates = Coordinate {
          x: old_coordinates.x,
          y: old_coordinates.y + step.substring(1, step.len()).parse::<i32>().unwrap(),
        };
        let path = (old_coordinates.clone(), new_coordinates.clone());
        old_coordinates = new_coordinates;
        path
      }
      _ => unreachable!(),
    })
    .collect()
}

// As inspired by https://stackoverflow.com/a/1968345
fn find_intersection(
  line_1: &(Coordinate, Coordinate),
  line_2: &(Coordinate, Coordinate),
) -> Option<Coordinate> {
  let s1_x = (line_1.1.x - line_1.0.x) as f32; // Come up with a better name for s
  let s1_y = (line_1.1.y - line_1.0.y) as f32;

  let s2_x = (line_2.1.x - line_2.0.x) as f32;
  let s2_y = (line_2.1.y - line_2.0.y) as f32;

  let s = (-s1_y * (line_1.0.x - line_2.0.x) as f32 + s1_x * (line_1.0.y - line_2.0.y) as f32) // Come up with a better name for s
    / (-s2_x * s1_y + s1_x * s2_y);
  let t = (s2_x * (line_1.0.y - line_2.0.y) as f32 - s2_y * (line_1.0.x - line_2.0.x) as f32) // Come up with a better name for t
    / (-s2_x * s1_y + s1_x * s2_y);

  if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
    let intersection_point = Coordinate {
      x: line_1.0.x + (t * s1_x).round() as i32,
      y: line_1.0.y + (t * s1_y).round() as i32,
    };
    if intersection_point != (Coordinate { x: 0, y: 0 }) {
      return Some(intersection_point);
    }
  }
  None
}

fn find_all_intersecting_points(
  route1: Vec<(Coordinate, Coordinate)>,
  route2: Vec<(Coordinate, Coordinate)>,
) -> Vec<Coordinate> {
  let mut intersections = Vec::new();
  for first_path in route1 {
    for second_path in &route2 {
      if let Some(coordinate) = find_intersection(&first_path, &second_path) {
        intersections.push(coordinate);
      }
    }
  }
  intersections
}

fn min_steps_to_intersection(
  route1: Vec<(Coordinate, Coordinate)>,
  route2: Vec<(Coordinate, Coordinate)>,
) -> i32 {
  let mut steps_1 = 0;
  let mut min_steps = std::i32::MAX;
  for first_path in route1 {
    let mut steps_2 = 0;
    for second_path in &route2 {
      if let Some(coordinate) = find_intersection(&first_path, &second_path) {
        min_steps = min_steps.min(
          steps_1
            + steps_2
            + first_path.0.manhattan_distance(coordinate.clone())
            + second_path.0.manhattan_distance(coordinate),
        );
      }

      steps_2 += second_path
        .0
        .clone()
        .manhattan_distance(second_path.1.clone());
    }
    steps_1 += first_path
      .0
      .clone()
      .manhattan_distance(first_path.1.clone());
  }
  min_steps
}

pub fn part_one(movement_1: &[String], movement_2: &[String]) -> i32 {
  let path_1 = movements_to_paths(movement_1);
  let path_2 = movements_to_paths(movement_2);
  let intersections = find_all_intersecting_points(path_1, path_2);
  let base = Coordinate { x: 0, y: 0 };
  intersections
    .iter()
    .map(|intersection| intersection.manhattan_distance(base.clone()))
    .min()
    .unwrap()
}

pub fn part_two(movement_1: &[String], movement_2: &[String]) -> i32 {
  let path_1 = movements_to_paths(movement_1);
  let path_2 = movements_to_paths(movement_2);
  min_steps_to_intersection(path_1, path_2)
}

#[cfg(test)]
pub mod test {
  use super::{
    find_all_intersecting_points, find_intersection, min_steps_to_intersection, movements_to_paths,
    part_one, Coordinate, ManhattanDistance,
  };

  const PATH: [(Coordinate, Coordinate); 9] = [
    (Coordinate { x: 0, y: 0 }, Coordinate { x: 75, y: 0 }),
    (Coordinate { x: 75, y: 0 }, Coordinate { x: 75, y: 30 }),
    (Coordinate { x: 75, y: 30 }, Coordinate { x: 158, y: 30 }),
    (Coordinate { x: 158, y: 30 }, Coordinate { x: 158, y: -53 }),
    (Coordinate { x: 158, y: -53 }, Coordinate { x: 146, y: -53 }),
    (Coordinate { x: 146, y: -53 }, Coordinate { x: 146, y: -4 }),
    (Coordinate { x: 146, y: -4 }, Coordinate { x: 217, y: -4 }),
    (Coordinate { x: 217, y: -4 }, Coordinate { x: 217, y: -11 }),
    (Coordinate { x: 217, y: -11 }, Coordinate { x: 145, y: -11 }),
  ];
  const INPUT_1_PATH1: [&str; 9] = ["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
  const INPUT_1_PATH2: [&str; 8] = ["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];
  const INPUT_2_PATH1: [&str; 11] = [
    "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
  ];
  const INPUT_2_PATH2: [&str; 10] = [
    "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
  ];

  #[test]
  fn day3_manhattan_distance() {
    let base = Coordinate { x: 0, y: 0 };
    let test_case = Coordinate { x: 3, y: 3 };
    assert_eq!(
      test_case.manhattan_distance(base.clone()),
      base.manhattan_distance(test_case.clone())
    );
    assert_eq!(test_case.manhattan_distance(base), 6)
  }

  #[test]
  fn day3_find_intersecion() {
    let p1_start = Coordinate { x: -2, y: 1 };
    let p1_end = Coordinate { x: 2, y: 1 };

    let p2_start = Coordinate { x: 0, y: -2 };
    let p2_end = Coordinate { x: 0, y: 2 };

    let p3_start = Coordinate { x: 3, y: -2 };
    let p3_end = Coordinate { x: 3, y: 2 };

    assert_eq!(
      find_intersection(&(p1_start.clone(), p1_end.clone()), &(p2_start, p2_end)),
      Some(Coordinate { x: 0, y: 1 })
    );
    assert_eq!(
      find_intersection(&(p1_start, p1_end), &(p3_start, p3_end)),
      None
    );
  }
  #[test]
  fn day3_find_all_intersections() {
    let p1_start = Coordinate { x: -2, y: 1 };
    let p1_end = Coordinate { x: 2, y: 1 };

    let p2_start = Coordinate { x: 0, y: -2 };
    let p2_end = Coordinate { x: 0, y: 2 };

    let p3_start = Coordinate { x: 3, y: -2 };
    let p3_end = Coordinate { x: 3, y: 2 };

    let path1 = [(p1_start, p1_end), (p3_start, p3_end)];
    let path2 = [(p2_start, p2_end)];
    assert_eq!(
      find_all_intersecting_points(path1.to_vec(), path2.to_vec()),
      &[Coordinate { x: 0, y: 1 }]
    );
    assert_eq!(
      find_all_intersecting_points(path1.to_vec(), Vec::new()),
      Vec::new()
    );
  }

  #[test]
  fn day3_find_all_intersections_2() {
    let input_1: Vec<String> = ["R8", "U5", "L5", "D3"]
      .iter()
      .map(|item| item.to_string())
      .collect();
    let input_2: Vec<String> = ["U7", "R6", "D4", "L4"]
      .iter()
      .map(|item| item.to_string())
      .collect();
    let movement_1 = movements_to_paths(&input_1);
    let movement_2 = movements_to_paths(&input_2);
    assert_eq!(
      find_all_intersecting_points(movement_1, movement_2),
      [Coordinate { x: 6, y: -5 }, Coordinate { x: 3, y: -3 }]
    )
  }

  #[test]
  fn day3_find_min_steps_simple() {
    let input_1: Vec<String> = ["R8", "U5", "L5", "D3"]
      .iter()
      .map(|item| item.to_string())
      .collect();
    let input_2: Vec<String> = ["U7", "R6", "D4", "L4"]
      .iter()
      .map(|item| item.to_string())
      .collect();
    let movement_1 = movements_to_paths(&input_1);
    let movement_2 = movements_to_paths(&input_2);
    assert_eq!(min_steps_to_intersection(movement_1, movement_2), 30)
  }

  #[test]
  fn day3_find_min_steps() {
    let input_1_path1: Vec<String> = INPUT_1_PATH1.iter().map(|item| item.to_string()).collect();
    let input_1_path2: Vec<String> = INPUT_1_PATH2.iter().map(|item| item.to_string()).collect();

    let input_2_path1: Vec<String> = INPUT_2_PATH1.iter().map(|item| item.to_string()).collect();
    let input_2_path2: Vec<String> = INPUT_2_PATH2.iter().map(|item| item.to_string()).collect();

    let mut movement_1 = movements_to_paths(&input_1_path1);
    let mut movement_2 = movements_to_paths(&input_1_path2);
    assert_eq!(min_steps_to_intersection(movement_1, movement_2), 610);

    movement_1 = movements_to_paths(&input_2_path1);
    movement_2 = movements_to_paths(&input_2_path2);

    assert_eq!(min_steps_to_intersection(movement_1, movement_2), 410)
  }

  #[test]
  fn day3_build_coordinates() {
    let movements: Vec<String> = INPUT_1_PATH1.iter().map(|item| item.to_string()).collect();
    assert_eq!(movements_to_paths(&movements), PATH);
    assert_eq!(movements_to_paths(&[]), []);
  }

  #[test]
  fn day3_part_one() {
    let movements_1: Vec<String> = INPUT_1_PATH1.iter().map(|item| item.to_string()).collect();
    let movements_2: Vec<String> = INPUT_1_PATH2.iter().map(|item| item.to_string()).collect();

    let movements_3: Vec<String> = INPUT_2_PATH1.iter().map(|item| item.to_string()).collect();
    let movements_4: Vec<String> = INPUT_2_PATH2.iter().map(|item| item.to_string()).collect();
    assert_eq!(part_one(&movements_1, &movements_2), 159);
    assert_eq!(part_one(&movements_3, &movements_4), 135);
  }
}
