extern crate advent_day2;

use std::fs::File;

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let mut data = advent_day2::parse_input(file);

    let checksum = advent_day2::checksum(&data);
    println!("Checksum: {}", checksum);

    let divide = advent_day2::divide(&mut data);
    println!("Divide: {}", divide);
}
