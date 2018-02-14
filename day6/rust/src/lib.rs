use std::collections::HashSet;

pub fn count_redistributions(mut memory_banks: Vec<i32>) -> (i32, i32) {
  let mut set = HashSet::new();
  let mut counter = 0;

  while !set.contains(&memory_banks) {
    // println!("{:?}", memory_banks);
    set.insert(memory_banks.clone());
    cycle(&mut memory_banks);
    counter += 1;
  }

  println!("{:?}", memory_banks);

  let target = memory_banks.clone();

  let mut loop_counter = 1;
  cycle(&mut memory_banks);

  while memory_banks != target {
    cycle(&mut memory_banks);
    loop_counter += 1;
  }

  (counter, loop_counter)
}

fn cycle(vec: &mut Vec<i32>) {
  let max = *vec.iter().max().unwrap();
  let max_index = vec.iter().position(|elem| *elem == max).unwrap();
  let len = vec.len();
  let mut carry = max % len as i32;
  vec[max_index] = 0;
  for i in (max_index + 1)..(max_index + 1 + len) {
    if carry > 0 {
      vec[i % len] += max / (len as i32) + 1;
      carry -= 1;
    } else {
      vec[i % len] += max / (len as i32);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple_redistribution() {
    assert_eq!(count_redistributions(vec![0, 2, 7, 0]), 5);
  }

  #[test]
  fn complex_redistribution() {
    assert_eq!(count_redistributions(vec![11, 11, 13, 7, 0, 15, 5, 5, 4, 4, 1, 1, 7, 1, 15, 11]), 4074);
  }
}