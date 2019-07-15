use crate::point::{Point};
use crate::bounding_box::BoundingBox;
use std::cmp::Ordering;

pub struct RTree {
  root: Option<RNode>,
  dims: usize,
  size: usize,
  max_rect_per_lvl: usize,
  max_entries_per_rect: usize,
}

impl RTree {
  pub fn new(dims: usize) -> Self {
    RTree{
      root: None,
      dims: dims,
      size: 0,
      max_rect_per_lvl: 9, // good constant?
      max_entries_per_rect: 10, // arbitrary, should fix
    }
  }
  pub fn add(&mut self, p: Point) {
    self.size += 1;
    match &mut self.root {
      None => self.root = Some(RNode::new_degenerate(p)),
      Some(ref mut r) => { r.add(p, self.max_entries_per_rect, self.max_rect_per_lvl); },
    }
  }
}

pub enum RNode {
  Internal(BoundingBox, Vec<Box<RNode>>),
  Leaf(BoundingBox, Vec<Point>),
}

impl RNode {
  fn new_degenerate(p: Point) -> Self {
    RNode::Leaf(BoundingBox::just(&p), vec!(p))
  }
  fn bb(&self) -> &BoundingBox {
    match &self {
      RNode::Internal(bb, _) | RNode::Leaf(bb, _) => bb,
    }
  }
  // propogates new nodes up the tree so they can be replaced if necessary
  fn add(&mut self, p: Point, max_entries: usize, max_rects: usize) -> Option<Vec<RNode>> {
    match self {
      RNode::Internal(ref mut bb, ref mut children) => {
        if !bb.contains(&p) { assert!(bb.expand_to(&p)) };
        let inserted = children.iter_mut()
          .map(|c| (c.bb().dist(&p), c))
          .min_by(|(d, _), (o_d, _)| (d.partial_cmp(o_d).unwrap_or(Ordering::Greater)))
          .map(|(_, child)| child)
          .expect("There was an internal node with no children");
        inserted.add(p, max_entries, max_rects);
          // TODO handle here
          unimplemented!();
      },
      RNode::Leaf(ref mut bb, ref mut pts) => {
        if !bb.contains(&p) { assert!(bb.expand_to(&p)) };
        pts.push(p);
        if pts.len() >= max_entries {
          unimplemented!()
          // TODO find dimension with most variance
        } else { None }
      },
    }
  }
}
