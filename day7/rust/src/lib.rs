#[derive(Debug, Clone)]
pub enum Tree<T, W> {
  Node {
    nodes: Vec<Tree<T, W>>,
    value: T,
    weight: W,
  },
  Leaf {
    value: T,
    weight: W,
  },
  Empty
}

impl<T, W> Tree<T, W> {
  pub fn add(self, node: Tree<T, W>) -> Tree<T, W> {
    use Tree::*;
    match self {
      Node {
        mut nodes,
        value,
        weight,
      } => {
        nodes.push(node);
        Node{ nodes: nodes, value: value, weight: weight }
      }
      Leaf {
        value,
        weight,
      } => Node {
        nodes: vec![node],
        value: value,
        weight: weight,
      },
      Empty => node
    }
  }
  pub fn set_values(self, value: T, weight: W) -> Tree<T, W> {
    use Tree::*;
    match self {
      Node { nodes, value: _, weight: _ } => {
        Node { nodes, value, weight }
      },
      Leaf { value: _, weight: _ } => {
        Leaf { value: value, weight: weight }
      },
      Empty => {
        Leaf { value: value, weight: weight }
      }
    }
  }
}
