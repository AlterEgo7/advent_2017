extern crate regex;
extern crate matches;
use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::rc::Rc;
use std::str::Split;
use Tree::*;

pub trait Item: Clone + Eq + Hash + fmt::Debug {}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Tree<V: Item, W: Item> (Rc<Node<V, W>>);

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Node<V: Item, W: Item> {
  Internal {
    value: V,
    weight: W,
    children: Vec<Tree<V, W>>
  },
  Leaf {
    value: V,
    weight: W
  }
}

type Forest<V, W> = Vec<Tree<V, W>>;

#[derive(Debug, Clone)]
pub struct NodeIndex<V: Item, W: Item> {
  map: HashMap<V, Rc<Node<V, W>>>,
  forest: Forest<V, W>,
}

impl<V, W> NodeIndex<V, W>
where
  V: Clone + Eq + Hash,
  W: Clone,
{
  pub fn new() -> NodeIndex<V, W> {
    NodeIndex {
      map: HashMap::new(),
      forest: Forest::new(),
    }
  }
}

pub fn parse_tree(input: String) -> Tree<String, u32> {
  unimplemented!()
}

fn parse_node(input: String, node_map: &mut NodeIndex<String, u32>) -> Rc<Tree<String, u32>> {
  let re = Regex::new(r"(\w+)\s\((\d+)\)(\s->\s)?(.+)?").unwrap();
  let captures = re.captures(&input).unwrap();

  match (captures.get(1), captures.get(2), captures.get(4)) {
    (Some(name), Some(weight), None) => create_node(name.as_str().to_string(), weight.as_str().parse::<u32>().unwrap(), None, node_map),
    (Some(name), Some(weight), Some(node_string)) => {
      let value = name.as_str().to_string();
      let weight = weight.as_str().parse::<u32>().unwrap();
      let children = Some(node_string.as_str().split(", "));
      create_node(value, weight, children, node_map)
    },
    _ => panic!("parse error!"),
  }
}

fn create_nodes(input: &str, node_map: &NodeIndex<String, u32>) -> Vec<Box<Tree<String, u32>>> {
  unimplemented!()
}

fn create_node(value: String, weight: u32, children: Option<Split<&str>>, node_map: &mut NodeIndex<String, u32>) -> Rc<Tree<String, u32>> {
  match children {
    Some(nodes) => {
      match node_map.map.entry(value.clone()) {
        Entry::Occupied(mut node_ref) => {
          Rc::get_mut(node_ref.get_mut()).unwrap().set_values(
            value,
            weight,
          );

          for child in nodes {}
        }
        Entry::Vacant(entry) => {}
      }

      unimplemented!()
    },
    None => match node_map.map.entry(value.clone()) {
      Entry::Occupied(mut node_ref) => {
        let mut node = node_ref.get_mut();
        (Rc::get_mut(node)).unwrap().set_values(value, weight);
        Rc::clone(node)
      }
      Entry::Vacant(entry) => {
        let new_node = Leaf { value, weight };
        let rc = Rc::new(new_node);
        entry.insert(Rc::clone(&rc));
        node_map.forest.push(Rc::clone(&rc));
        rc
      }
    },
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
