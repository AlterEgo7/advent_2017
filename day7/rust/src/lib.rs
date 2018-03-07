#![feature(slice_patterns)]
extern crate regex;
extern crate matches;
use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::fmt::Debug;
use std::rc::Rc;
use std::str::Split;
use Node::*;

pub type w_size = u32;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Tree<V: Clone + Eq + Hash + Debug> (Rc<Node<V>>);

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Node<V: Clone + Eq + Hash + Debug> {
  Internal {
    value: V,
    weight: w_size,
    children: Vec<Tree<V>>
  },
  Leaf {
    value: V,
    weight: w_size
  }
}

type Forest<V> = Vec<Tree<V>>;

#[derive(Debug, Clone)]
pub struct NodeIndex<V: Clone + Eq + Hash + Debug> {
  map: HashMap<V, Rc<Node<V>>>,
  forest: Forest<V>,
}

impl<V> NodeIndex<V>
where
  V: Clone + Eq + Hash + Debug
{
  pub fn new() -> Self {
    NodeIndex {
      map: HashMap::new(),
      forest: Forest::new(),
    }
  }
}

impl<'a, V> Tree<V> 
where
  V: Clone + Eq + Hash + Debug
{
  pub fn from_children(value: V, weight: w_size, children: Vec<Self>) -> Self {
    let element = if children.is_empty() {
      Leaf { value, weight }
    } else {
      Internal { value, weight, children }
    };

    Tree(Rc::new(element))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use matches::*;

  #[test]
  fn tree_set_values_empty() {
    let mut tree = Tree::Empty;
    tree.set_values("123", 42);
    assert_eq!(tree.value(), Some("123"));
    assert_eq!(tree.weight(), Some(42));
  }

  #[test]
  fn tree_set_values_leaf() {
    let mut tree = Tree::Leaf {
      value: "initial",
      weight: 42,
    };
    tree.set_values("123", 42);
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
    let node = &mut Tree::Node {
      value: "initial",
      weight: 42,
      nodes: vec![
        Box::new(Tree::Leaf {
          value: "test",
          weight: 7,
        }),
      ],
    };
    let new_tree = node.add(Box::new(Tree::Leaf {
      value: "test",
      weight: 2,
    }));
    assert_eq!(new_tree.nodes().unwrap().len(), 2);
  }

  #[test]
  fn parse_node_test() {
    let input = String::from("pbga (66)");
    let mut index = NodeIndex::new();
    let node = parse_node(input, &mut index);
    assert_eq!(node.value().unwrap(), "pbga");
    assert_eq!(node.weight().unwrap(), 66);
    assert!(index.map.contains_key("pbga"));
  }
}
