extern crate regex;
use regex::{Regex, Captures};
use std::collections::HashMap;
use Tree::*;

type Forest<V: Clone, W: Clone> = Vec<Box<Tree<V, W>>>;
struct NodeIndex<V: Clone, W: Clone> {
  map: HashMap<V, Box<Tree<V, W>>>,
  forest: Forest<V, W>
}

pub fn parse_tree(input: String) -> Tree<String, u32> {
  unimplemented!()
}

fn parse_node(input: String, node_map: NodeIndex<String, u32>) -> Tree<String, u32> {
  let re = Regex::new(r"(\w+)\s\((\d+)\)(\s->\s)?(.+)?").unwrap();
  let captures = re.captures(&input).unwrap();

  match (captures.get(1), captures.get(2), captures.get(4)) {
    (Some(name), Some(weight), None) => Leaf { value: name.as_str().to_string(), weight: weight.as_str().parse::<u32>().unwrap() },
    (Some(name), Some(weight), Some(node_string)) =>
      Node { value: name.as_str().to_string(), weight: weight.as_str().parse::<u32>().unwrap(), nodes: create_nodes(node_string.as_str(), node_map)},
    _ => panic!("parse error!")
  }
}

fn create_nodes(input: &str, node_map: NodeIndex<String, u32>) -> Vec<Box<Tree<String, u32>>> {
  unimplemented!()
}

#[derive(Debug, Clone)]
pub enum Tree<T: Clone, W: Clone> {
  Node {
    nodes: Vec<Box<Tree<T, W>>>,
    value: T,
    weight: W,
  },
  Leaf {
    value: T,
    weight: W,
  },
  Empty,
}

impl<T, W> Tree<T, W>
    where T: Clone, W: Clone
{
  pub fn add(self, node: Box<Tree<T, W>>) -> Tree<T, W> {
    use Tree::*;
    match self {
      Node {
        mut nodes,
        value,
        weight,
      } => {
        nodes.push(node);
        Node {
          nodes: nodes,
          value: value,
          weight: weight,
        }
      }
      Leaf { value, weight } => Node {
        nodes: vec![node],
        value: value,
        weight: weight,
      },
      Empty => *node.clone(),
    }
  }
  pub fn set_values(self, value: T, weight: W) -> Tree<T, W> {
    use Tree::*;
    match self {
      Node {
        nodes,
        value: _,
        weight: _,
      } => Node {
        nodes,
        value,
        weight,
      },
      Leaf {
        value: _,
        weight: _,
      } => Leaf {
        value: value,
        weight: weight,
      },
      Empty => Leaf {
        value: value,
        weight: weight,
      },
    }
  }
}
