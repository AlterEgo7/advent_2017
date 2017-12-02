use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn row_checksum(row: &Vec<i32>) -> i32 {
    let max = row.iter().max().unwrap();
    let min = row.iter().min().unwrap();

    (max - min)
}

fn row_divide(row: &mut Vec<i32>) -> i32 {
    row.sort();
    row.reverse();
    let mut res: i32 = -1;
    for (index, num) in row.iter().enumerate() {
        let mut reverse_iter = row[(index + 1)..].iter().rev();
        if let Some(divident) = reverse_iter.find(|&&elem| {
            num % elem == 0
        }) {
            res = num / divident;
        }
    };
    res
}

pub fn checksum(data: &Vec<Vec<i32>>) -> i32 {
    data.iter().map(|row| row_checksum(row)).sum()
}

pub fn divide(data: &mut Vec<Vec<i32>>) -> i32 {
    data.iter_mut().map(|row| row_divide(row)).sum()
}

pub fn parse_input(file: File) -> Vec<Vec<i32>> {
    let file = BufReader::new(&file);
    file.lines().map(|line| {
        let line = line.unwrap();
        line.split('\t').map(|s| {
            s.parse::<i32>().unwrap()
        }).collect()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checksum_test() {
        let input = vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];
        assert_eq!(checksum(input), 18);
    }

    #[test]
    fn divide_test() {
        let input = &mut vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]];
        assert_eq!(divide(input), 9)
    }
}