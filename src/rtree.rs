use crate::bounding_box::BoundingBox;

pub struct RTree {
  size: usize,

  pages: Vec<RPage>,
  page_depth: usize,
  dist: fn(&BoundingBox, &BoundingBox) -> f32,

  branches: Vec<RBranch>,
}

const MAX_PER_LVL: usize = 9;
const MAX_ENTRIES: usize = 100_000;
const ROOT: usize = 0;

pub struct RPage(BoundingBox, Vec<(BoundingBox, T)>);
pub struct RBranch {
  // depth of this node
  // will need to check if children are 1 + depth
  depth: usize,
  bb: BoundingBox,
  children: Vec<usize>,
}

impl RTree {
  pub fn new() -> Self {
    RTree{
      size: 0,
      pages: vec!(),
      page_depth: 1,
      branches: vec!(RBranch::new(0)),
    }
  }
  pub fn add(&mut self, item: (BoundingBox, T) {
    let mut curr = branches[ROOT];
    while curr.depth < self.page_depth + 1 {
      let next = curr.children.iter()
        .map(|c_index| self.branches[c_index])
        .map(|c| (c, self.dist(&c.bb, &item.0)))
        .min_by_key(|(_, d)| d);
      // TODO handle next being some(i.e. selected)
      // or none aka no children
      unimplemented!();
    };
    unimplemented!();
  }
}

impl RBranch {
  pub fn new(depth: usize) -> Self {
    let children = vec!();
    RBranch{depth, children}
  }
}
