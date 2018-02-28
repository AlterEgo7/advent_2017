extern crate matches;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::rc::Rc;
use Tree::*;

type Forest<V, W> = Vec<Rc<Tree<V, W>>>;

#[derive(Debug, Clone)]
pub struct NodeIndex<V: Clone + Eq + Hash, W: Clone> {
  map: HashMap<V, Rc<Tree<V, W>>>,
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
    (Some(name), Some(weight), None) => {
      let value = name.as_str().to_string();
      let weight = weight.as_str().parse::<u32>().unwrap();

      match node_map.map.entry(value.clone()) {
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
      }
    }
    (Some(name), Some(weight), Some(node_string)) => {
      match node_map.map.entry(name.as_str().to_string()) {
        Entry::Occupied(node) => {
          
        }
        Entry::Vacant(entry) => {
        }
      }

      unimplemented!()
    }
    _ => panic!("parse error!"),
  }
}

fn create_nodes(input: &str, node_map: &NodeIndex<String, u32>) -> Vec<Box<Tree<String, u32>>> {
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
      },
    }
  }

  pub fn set_values(&mut self, value: T, weight: W) {
    use Tree::*;
    match *self {
      Node {
        value: ref mut old_value,
        weight: ref mut old_weight,
        ..
      } => {
        *old_value = value;
        *old_weight = weight;
      }
      Leaf {
        value: ref mut old_value,
        weight: ref mut old_weight,
      } => {
        *old_value = value;
        *old_weight = weight;
      }
      Empty => {
        *self = Leaf {
          value: value,
          weight: weight,
        }
      }
    }
  }

  pub fn value(&self) -> Option<T> {
    match *self {
      Node { ref value, .. } => Some(value.clone()),
      Leaf { ref value, .. } => Some(value.clone()),
      Empty => None,
    }
  }

  pub fn weight(&self) -> Option<W> {
    match *self {
      Node { ref weight, .. } => Some(weight.clone()),
      Leaf { ref weight, .. } => Some(weight.clone()),
      Empty => None,
    }
  }

  pub fn nodes(&self) -> Option<&NodeList<T, W>> {
    match *self {
      Node { ref nodes, .. } => Some(&nodes),
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
