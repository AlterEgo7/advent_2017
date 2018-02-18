use std::fs::File;

extern crate day5;
use day5::*;

fn main() {
    let f = File::open("input.txt").unwrap();
    println!("Steps = {}", count_steps(parse(f)));
}

