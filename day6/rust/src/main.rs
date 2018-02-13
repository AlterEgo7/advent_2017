use std::fs::File;
use std::io::prelude::*;

extern crate advent6;
use advent6::*;

fn main() {
    let mut file = File::open("input").expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("error");

    let memory_banks: Vec<i32> = contents
        .trim()
        .split("\t")
        .map(|elem| elem.parse::<i32>().unwrap())
        .collect();
        
    println!("{:?}", memory_banks);
    println!("{:?}", count_redistributions(memory_banks));
}
