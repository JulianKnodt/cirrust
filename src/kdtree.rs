use std::cmp::Ordering;
use std::f32;
use crate::point::{Point, l2norm};

#[derive(Debug)]
pub struct KDTree {
  root: Option<KDNode>,
  dims: usize,
  size: usize,
}

impl KDTree {
  pub fn new(dims: usize) -> Self {
    KDTree{
      root: None,
      dims: dims,
      size: 0,
    }
  }
  pub fn from(_pts: Vec<Point>) -> Self {
    // https://math.stackexchange.com/questions/1764878/how-to-efficiently-create-balanced-kd-trees-from-a-static-set-of-points
    unimplemented!()
  }
  pub fn add(&mut self, v: Point) {
    self.size += 1;
    match &mut self.root {
      None => { self.root.replace(KDNode::new(v, 0)); },
      Some(ref mut r) => r.add(v, self.dims),
    }
  }
  pub fn nearest(&self, v: &Point) -> Option<Point> {
    self.root.as_ref().map(|r| r.nearest(v).0.clone())
  }
  pub fn size(&self) -> usize { self.size }
}

#[derive(Debug)]
struct KDNode {
  item: Point,
  cmp_dim: usize,
  l: Option<Box<KDNode>>,
  r: Option<Box<KDNode>>,
}

impl KDNode {
  fn new(v: Point, dim: usize) -> Self {
    KDNode{
      item: v, cmp_dim: dim,
      l: None, r: None,
    }
  }
  fn add(&mut self, v: Point, dims: usize) {
    let cmp = self.item[self.cmp_dim].partial_cmp(&v[self.cmp_dim]);
    let item = match cmp {
      Some(Ordering::Greater) | None => &mut self.r,
      Some(Ordering::Less) | Some(Ordering::Equal) => &mut self.l,
    };
    let next_dim = (self.cmp_dim + 1) % dims;
    match item {
      None => { item.replace(Box::new(KDNode::new(v, next_dim))); },
      Some(ref mut r) => r.add(v, dims),
    };
  }
  fn nearest(&self, v: &Point) -> (&Point, f32) {
    let self_dist = l2norm(&self.item, v);
    let (close, far) = match self.item[self.cmp_dim].partial_cmp(&v[self.cmp_dim]) {
      Some(Ordering::Greater) | None => (&self.r, &self.l),
      Some(Ordering::Less) | Some(Ordering::Equal) => (&self.l, &self.r),
    };
    let (near_pt, near_dist) = close.as_ref()
      .map(|child| child.nearest(v))
      .filter(|&(_, dist)| dist < self_dist)
      .unwrap_or_else(|| (&self.item, self_dist));

    let must_check_other_side = (near_pt[self.cmp_dim] - v[self.cmp_dim]).abs() < near_dist;
    far.as_ref()
      .filter(|_| must_check_other_side)
      .map(|far_side| far_side.nearest(v))
      .filter(|&(_, dist)| dist < near_dist)
      .unwrap_or((near_pt, near_dist))
  }
}

#[cfg(test)]
mod kdtree_test {
  use crate::kdtree::KDTree;
  #[test]
  fn test() {
    let mut t = KDTree::new(2);
    t.add(vec!(2.,3.));
    t.add(vec!(3.,2.));
    t.add(vec!(1.,1.5));
    t.add(vec!(1.,2.));
    assert_eq!(t.size(), 4);
    println!("{:?}", t);
    assert_eq!(t.nearest(&vec!(3.,2.)), Some(vec!(3., 2.)));
    assert_eq!(t.nearest(&vec!(0.,0.)), Some(vec!(1.,1.5)));
  }

  // TODO more tests
}
