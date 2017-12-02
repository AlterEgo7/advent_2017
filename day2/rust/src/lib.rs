use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn row_checksum(row: &Vec<i32>) -> u32 {
    let max = row.iter().max().unwrap();
    let min = row.iter().min().unwrap();

    (max - min) as u32
}

pub fn checksum(data: Vec<Vec<i32>>) -> u32 {
    data.iter().map(|row| row_checksum(row)).sum()
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
}