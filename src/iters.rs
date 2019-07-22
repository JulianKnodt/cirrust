use crate::kdtree::KDNode;

pub struct DepthFirst<I>(Vec<I>);
impl<I> DepthFirst<I> {
  fn new(v: I) -> Self { DepthFirst(vec!(v)) }
}

impl<'a> Iterator for DepthFirst<&'a KDNode> {
  type Item = &'a KDNode;
  fn next(&mut self) -> Option<Self::Item> {
    let next = self.0.pop();
    next.map(|n| n.children().rev().for_each(|c| self.0.push(c)));
    next
  }
}

use std::collections::VecDeque;
pub struct BreadthFirst<I>(VecDeque<I>);
impl<I> BreadthFirst<I> {
  fn new(v: I) -> Self {
    let mut buf = VecDeque::new();
    buf.push_front(v);
    BreadthFirst(buf)
  }
}

impl<'a> Iterator for BreadthFirst<&'a KDNode> {
  type Item = &'a KDNode;
  fn next(&mut self) -> Option<Self::Item> {
    let next = self.0.pop_front();
    next.map(|n| n.children().for_each(|c| self.0.push_back(c)));
    next
  }
}

