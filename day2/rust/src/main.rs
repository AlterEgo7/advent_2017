extern crate advent_day2;

use std::fs::File;

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();

    let checksum = advent_day2::checksum(advent_day2::parse_input(file));
    println!("Checksum: {}", checksum);
}
