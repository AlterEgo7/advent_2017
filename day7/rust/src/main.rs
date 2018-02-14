use std::fs::File;
use std::io::prelude::*;
extern crate regex;
extern crate day7;
use day7::*;
use regex::Regex;

fn main() {
    let mut file = File::open("input.txt").expect("not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("parsing error");

    let re = Regex::new(r"(\w+)\s\((\d+)\)(\s->\s)?(.+)?").unwrap();
    let capt = re.captures("fwft (72) -> ktlj, cntj, xhth").unwrap();
    

    println!("{:?}", capt);
}
