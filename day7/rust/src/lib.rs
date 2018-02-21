extern crate matches;
extern crate regex;
use regex::{Captures, Regex};
use std::collections::HashMap;
use Tree::*;

type Forest<V: Clone, W: Clone> = Vec<Box<Tree<V, W>>>;
struct NodeIndex<V: Clone, W: Clone> {
  map: HashMap<V, Box<Tree<V, W>>>,
  forest: Forest<V, W>,
}

pub fn parse_tree(input: String) -> Tree<String, u32> {
  unimplemented!()
}

fn parse_node(input: String, node_map: NodeIndex<String, u32>) -> Tree<String, u32> {
  let re = Regex::new(r"(\w+)\s\((\d+)\)(\s->\s)?(.+)?").unwrap();
  let captures = re.captures(&input).unwrap();

  match (captures.get(1), captures.get(2), captures.get(4)) {
    (Some(name), Some(weight), None) => Leaf {
      value: name.as_str().to_string(),
      weight: weight.as_str().parse::<u32>().unwrap(),
    },
    (Some(name), Some(weight), Some(node_string)) => Node {
      value: name.as_str().to_string(),
      weight: weight.as_str().parse::<u32>().unwrap(),
      nodes: create_nodes(node_string.as_str(), node_map),
    },
    _ => panic!("parse error!"),
  }
}

fn create_nodes(input: &str, node_map: NodeIndex<String, u32>) -> Vec<Box<Tree<String, u32>>> {
  unimplemented!()
}

type NodeList<T, W> = Vec<Box<Tree<T, W>>>;
#[derive(Debug, Clone)]
pub enum Tree<T: Clone, W: Clone> {
  Node {
    nodes: NodeList<T, W>,
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
where
  T: Clone,
  W: Clone,
{
  pub fn add(&mut self, node: Box<Tree<T, W>>) -> Tree<T, W> {
    use Tree::*;

    match *node {
      Empty => self.clone(),
      _ => match self {
        &mut Node {
          ref mut nodes,
          ref value,
          ref weight,
        } => {
          nodes.push(node);
          Node {
            nodes: nodes.clone(),
            value: value.clone(),
            weight: weight.clone(),
          }
        }
        &mut Leaf {
          ref value,
          ref weight,
        } => Node {
          nodes: vec![node],
          value: value.clone(),
          weight: weight.clone(),
        },
        &mut Empty => *node.clone(),
      }
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

  pub fn value(&self) -> Option<T> {
    match *self {
      Node {
        ref value,
        weight: _,
        nodes: _,
      } => Some(value.clone()),
      Leaf {
        ref value,
        weight: _,
      } => Some(value.clone()),
      Empty => None,
    }
  }

  pub fn weight(&self) -> Option<W> {
    match *self {
      Node {
        value: _,
        ref weight,
        nodes: _,
      } => Some(weight.clone()),
      Leaf {
        value: _,
        ref weight,
      } => Some(weight.clone()),
      Empty => None,
    }
  }

  pub fn nodes(&self) -> Option<&NodeList<T, W>> {
    match *self {
      Node {
        value: _,
        weight: _,
        ref nodes,
      } => Some(&nodes),
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use matches::*;

  #[test]
  fn tree_set_values_empty() {
    let tree = Tree::Empty.set_values("123", 42);
    assert_eq!(tree.value(), Some("123"));
    assert_eq!(tree.weight(), Some(42));
  }

  #[test]
  fn tree_set_values_leaf() {
    let tree = Tree::Leaf {
      value: "initial",
      weight: 42,
    }.set_values("123", 42);
    assert_eq!(tree.value(), Some("123"));
    assert_eq!(tree.weight(), Some(42));
  }

  #[test]
  fn tree_add_node_leaf() {
    let mut tree = &mut Tree::Leaf {
      value: "initial",
      weight: 42,
    };
    let new_tree = tree.add(Box::new(Tree::Leaf {
      value: "test",
      weight: 32,
    }));
    assert!(matches!(new_tree, Tree::Node{value, weight, ref nodes}));
    assert_eq!(new_tree.nodes().unwrap().len(), 1);
  }

  #[test]
  fn tree_add_node_node() {
    let mut node = &mut Tree::Node {
      value: "initial",
      weight: 42,
      nodes: vec![
        Box::new(Tree::Leaf {
          value: "test",
          weight: 7,
        }),
      ],
    };
    let new_tree = node.add(Box::new(Tree::Leaf{value: "test", weight: 2}));
    assert_eq!(new_tree.nodes().unwrap().len(), 2);
  }
}
