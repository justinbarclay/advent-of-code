pub fn parse_input(input: &str) -> Vec<usize> {
  input
    .trim()
    .split(',')
    .map(|num| num.parse::<usize>().unwrap())
    .collect()
}

#[derive(PartialEq)]
enum OpCode {
  Add,
  Multiply,
  Halt,
}

impl From<usize> for OpCode {
  fn from(op: usize) -> OpCode {
    match op {
      1 => OpCode::Add,
      2 => OpCode::Multiply,
      99 => OpCode::Halt,
      _ => unreachable!(),
    }
  }
}

fn run(register: &[usize], noun: Option<usize>, verb: Option<usize>) -> Vec<usize> {
  let mut new_register: Vec<usize> = register.to_vec();
  // Swap positions as specified in instructions
  if noun.is_some() && verb.is_some() {
    new_register[1] = noun.unwrap();
    new_register[2] = verb.unwrap();
  }

  let mut position = 0usize;
  loop {
    if position >= new_register.len() - 3{
      break;
    }
    let op: OpCode = new_register[position].into();

    let pos1 = new_register[position + 1];
    let pos2 = new_register[position + 2];
    let pos3 = new_register[position + 3];

    match op {
      OpCode::Add => {
        new_register[pos3] = new_register[pos1] + new_register[pos2];
      }
      OpCode::Multiply => {
        new_register[pos3] = new_register[pos1] * new_register[pos2];
      }
      OpCode::Halt => break,
    }
    position += 4;
  }
  new_register
}

pub fn part_one(register: &[usize]) -> Vec<usize> {
  run(register, Some(12), Some(2))
}

pub fn part_two(register: &[usize], matches: usize) -> (usize, usize) {
  for noun in 0..100 {
    for verb in 0..100 {
      let items = run(register, Some(noun), Some(verb));
      let value = items[0];
      if value == matches {
        return (noun, verb);
      }
    }
  }
  (0, 0)
}

#[cfg(test)]
pub mod test {
  use super::{part_two, run};

  #[test]
  fn day2_part_run() {
    assert_eq!(run(&[1, 0, 0, 0, 99], None, None), [2, 0, 0, 0, 99]);
    assert_eq!(run(&[2, 3, 0, 3, 99], None, None), [2, 3, 0, 6, 99]);
    assert_eq!(
      run(&[2, 4, 4, 5, 99, 0], None, None),
      [2, 4, 4, 5, 99, 9801]
    );
    assert_eq!(
      run(&[1, 1, 1, 4, 99, 5, 6, 0, 99], None, None),
      [30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
  }

  #[test]
  fn day2_part_two() {
    assert_eq!(part_two(&[1, 0, 0, 0, 99], 100), (0, 4));
  }
}
