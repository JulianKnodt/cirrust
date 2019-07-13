use crate::point::Point;
use crate::bounding_box::BoundingBox;

pub struct RTree {
  root: Option<RNode>,
  dims: usize,
  size: usize,
  rect_per_lvl: usize,
  max_entries_per_rect: usize,
}

impl RTree {
  pub fn new(dims: usize) -> Self {
    RTree{
      root: None,
      dims: dims,
      pts: 0,
      rect_per_level: 9, // good constant?
      max_entries_per_rect: 10, // arbitrary, should fix
    }
  }
  pub fn add(&mut self, p: Point) {
    self.size += 1;
    match &mut self.root {
      None => self.root = Some(p),
      Some(ref mut r) => r.add(p),
    }
  }
}

pub enum RNode {
  Internal(BoundingBox, Vec<Option<Box<RNode>>>),
  Leaf(BoundingBox, Vec<Point>),
}

impl RNode {
  fn add(&mut self, p: Point) {
    match &mut self {
      Internal(bb, children) => {

      },
      Leaf(bb, pts) => {
      
        pts.push(p);
      },
    }
  }
}
