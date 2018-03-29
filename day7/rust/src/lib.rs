#![feature(slice_patterns)]
extern crate regex;
extern crate matches;
use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::hash::Hash;
use std::fmt::Debug;
use std::rc::Rc;
use std::str::Split;
use Node::*;

pub type WSize = u32;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Tree<V: Clone + Eq + Hash + Debug> (Rc<Node<V>>);

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Node<V: Clone + Eq + Hash + Debug> {
  Internal {
    value: V,
    weight: WSize,
    children: Vec<Tree<V>>
  },
  Leaf {
    value: V,
    weight: WSize
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
  pub fn new(value: V, weight: WSize) -> Self {
    Self::from_children(value, weight, vec![])
  }

  pub fn from_children(value: V, weight: WSize, children: Vec<Self>) -> Self {
    let element = if children.is_empty() {
      Leaf { value, weight }
    } else {
      Internal { value, weight, children }
    };

    Tree(Rc::new(element))
  }

  pub fn value(&self) -> &V {
    match self.0.as_ref() {
        &Internal { ref value, .. } => value,
        &Leaf { ref value, .. } => value,
    }
  }

    pub fn weight(&self) -> &WSize {
      match self.0.as_ref() {
          &Internal { ref weight, .. } => weight,
          &Leaf { ref weight, .. } => weight,
      }
    }

    pub fn children(&self) -> Option<&Vec<Tree<V>>> {
      match self.0.as_ref() {
          &Internal { ref children, .. } => Some(children),
          &Leaf { .. } => None,
    }
  }
}

pub fn create_leaf<V: Clone + Eq + Hash + Debug>(value: V, new_weight: WSize, mut node_index: NodeIndex<V>) {
  match node_index.map.entry(value.clone()) {
    Vacant(entry) => {
      let tree = Tree::new(value, new_weight);
      entry.insert(Rc::clone(&tree.0));
      node_index.forest.push(tree);
    }
    Occupied(mut node) => match Rc::get_mut(&mut node.get_mut()).unwrap() {
      &mut Internal { ref mut weight, .. } => *weight = new_weight,
      &mut Leaf { ref mut weight, .. } => *weight = new_weight
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use matches::*;

  #[test]
  fn tree_new() {
    let tree = Tree::new("test", 42);
    assert_eq!(*tree.value(), "test");
    assert!(*tree.weight() == 42);
  }

  #[test]
  fn test_from_children() {
    let children = vec![Tree::new("child1", 1)];
    let tree = Tree::from_children("test", 42, children);
    let child = &tree.children().unwrap()[0];
    assert!(*child.weight() == 1);
  }
}
