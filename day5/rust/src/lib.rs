use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn count_steps(mut instructions: Vec<i32>) -> i32 {
    let mut index: usize = 0;
    let mut steps = 0;

    while index < instructions.len() {
        let jump_offset: i32 = instructions[index];
        if jump_offset >= 3 {
            instructions[index] -= 1;
        } else {
            instructions[index] += 1;
        }
        index = (index as i32 + jump_offset) as usize;
        steps += 1;
    }
    steps
}

pub fn parse(f: File) -> Vec<i32> {
    let file = BufReader::new(f);
    let mut res: Vec<i32> = vec![];
    for line in file.lines() {
        res.push(line.unwrap().parse::<i32>().unwrap())
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_steps() {
        let mut input = vec![0, 3, 0, 1, -3];
        assert_eq!(count_steps(&mut input), 5);
    }
}
