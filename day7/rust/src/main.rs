use std::fs::File;
use std::io::prelude::*;

extern crate day7;
use day7::*;

fn main() {
    let mut file = File::open("input.txt").expect("not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("parsing error");

    let mut tree: Tree<&str, u32> = Tree::Empty;
    let node: Tree<&str, u32> = Tree::Leaf{value: "node", weight: 2};
    tree = tree.set_values("test", 1);
    tree = tree.add(node);

    println!("{:?}", tree);
}
