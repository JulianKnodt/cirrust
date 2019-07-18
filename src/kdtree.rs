use std::{
  cmp::Ordering,
  f32,
};
use crate::point::{Point};

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
  pub fn is_empty(&self) -> bool {
    self.root.is_some()
  }
  pub fn remove(&mut self, p: &Point) -> bool {
    let remove_root = self.root.as_ref().map_or(false, |r| &r.item == p);
    if remove_root {
      let mut root = self.root.as_mut().unwrap();
      let cmp_dim = root.cmp_dim;
      let replacement = root.r.as_mut()
        .map(|r| (r.find_min(cmp_dim).clone(), true))
        .or_else(||
          root.l.as_mut().map(|l| (l.find_max(cmp_dim).clone(), false))
        );
      match replacement {
        None => assert!(self.root.take().is_some()),
        Some((repl_point, go_r)) => {
          root.item = repl_point;
          assert!((if go_r { root.r.as_mut() } else { root.l.as_mut() })
            .unwrap()
            .remove(&repl_point));
        },
      };
      return true;
    }
    self.root.as_mut().map_or(false, |r| r.remove(p))
  }
  pub fn contains(&self, v: &Point) -> bool {
    assert_eq!(v.len(), self.dims);
    self.root.as_ref().map_or(false, |r| r.contains(v))
  }
  pub fn nearest(&self, v: &Point) -> Option<&Point> {
    self.root.as_ref().map(|r| r.nearest(v).0)
  }
  pub fn find_max(&self, d: usize) -> Option<&Point> {
    self.root.as_ref().map(|r| r.find_max(d))
  }
  pub fn find_min(&self, d: usize) -> Option<&Point> {
    self.root.as_ref().map(|r| r.find_min(d))
  }
  pub fn size(&self) -> usize { self.size }
}

#[derive(Debug)]
struct KDNode {
  item: Point,
  cmp_dim: usize,
  // l is lesser
  l: Option<Box<KDNode>>,
  // right is greater
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
  fn is_leaf(&self) -> bool { self.l.is_none() && self.r.is_none() }
  fn contains(&self, v: &Point) -> bool {
    if &self.item == v { return true };
    let cmp = self.item[self.cmp_dim].partial_cmp(&v[self.cmp_dim]);
    match cmp {
      Some(Ordering::Greater) | None => &self.r,
      Some(Ordering::Less) | Some(Ordering::Equal) => &self.l,
    }.as_ref().map_or(false, |c| c.contains(v))
  }
  fn remove(&mut self, v: &Point) -> bool {
    let cmp = self.item[self.cmp_dim].partial_cmp(&v[self.cmp_dim]);
    let _next = match cmp {
      Some(Ordering::Greater) | None => &mut self.r,
      Some(Ordering::Less) | Some(Ordering::Equal) => &mut self.l,
    };
    unimplemented!()
  }
  fn find_min(&self, d: usize) -> &Point {
    let l = self.l.as_ref().filter(|_| self.cmp_dim != d).map(|l| l.find_min(d));
    let r = self.r.as_ref().map(|r| r.find_min(d));
    match (r, l) {
      (None, None) => &self.item,
      (Some(v), None) | (None, Some(v)) => v,
      (Some(r), Some(l)) => if r[d] < l[d] { r }
        else if self.item[d] < l[d] { &self.item }
        else { l },
    }
  }
  fn find_max(&self, d: usize) -> &Point {
    let r = self.r.as_ref().filter(|_| self.cmp_dim != d).map(|r| r.find_max(d));
    let l = self.l.as_ref().map(|l| l.find_max(d));
    match (r, l) {
      (None, None) => &self.item,
      (Some(v), None) | (None, Some(v)) => v,
      (Some(r), Some(l)) => if r[d] > l[d] { r }
        else if self.item[d] > l[d] { &self.item }
        else { l },
    }
  }
  fn nearest(&self, v: &Point) -> (&Point, f32) {
    let self_dist = self.item.dist(v);
    let (close, far) = match self.item[self.cmp_dim].partial_cmp(&v[self.cmp_dim]) {
      Some(Ordering::Greater) | None => (&self.r, &self.l),
      Some(Ordering::Less) | Some(Ordering::Equal) => (&self.l, &self.r),
    };
    let (near_pt, near_dist) = close.as_ref()
      .map(|child| child.nearest(v))
      .filter(|&(_, dist)| dist < self_dist)
      .unwrap_or_else(|| (&self.item, self_dist));
    far.as_ref()
      // only check other side if current item is within hypersphere
      .filter(|_| (self.item[self.cmp_dim] - v[self.cmp_dim]).abs() < near_dist)
      .map(|far_side| far_side.nearest(v))
      .filter(|&(_, dist)| dist < near_dist)
      .unwrap_or((near_pt, near_dist))
  }
}

#[cfg(test)]
mod kdtree_test {
  use crate::kdtree::KDTree;
  use crate::point::Point;
  fn naive_nearest<'a>(v: &'a Vec<Point>, o: &Point) -> &'a Point {
    assert!(!v.is_empty());
    v.iter()
      .map(|v| (v, v.dist(o)))
      .min_by(|(_, d), (_, o_d)| d.partial_cmp(&o_d).unwrap())
      .unwrap().0
  }
  fn naive_max<'a>(v: &'a Vec<Point>, d: usize) -> &'a Point {
    assert!(!v.is_empty());
    v.iter().max_by(|a, b| a[d].partial_cmp(&b[d]).unwrap()).unwrap()
  }
  fn naive_min<'a>(v: &'a Vec<Point>, d: usize) -> &'a Point {
    assert!(!v.is_empty());
    v.iter().min_by(|a, b| a[d].partial_cmp(&b[d]).unwrap()).unwrap()
  }
  // Just a bad implementation of random for testing
  // Didn't want to bring in the crate just for that
  struct BadRand(i64);
  const M: i64 = 1i64 << 31;
  const A: i64 = 1103515245;
  const C: i64 = 12345;
  impl BadRand {
    pub fn new() -> Self {
      use std::time::{SystemTime};
      let t = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
      BadRand(t as i64)
    }
    pub fn v(&mut self, m: i64) -> i64 {
      self.0 = (self.0 * A + C) % M;
      return self.0 % m;
    }
  }
  #[test]
  fn all_test() {
    let mut t = KDTree::new(2);
    let items: Vec<_> = vec!((2.,3.), (3.,2.), (1.,1.5), (1.,2.)).iter()
      .map(|&(a,b)| Point::from((a,b,0.)))
      .collect();
    items.iter().for_each(|p| t.add(p.clone()));
    assert_eq!(t.size(), 4);
    assert_eq!(t.nearest(&Point::from(&vec!(3.,2.))), Some(&Point::from(&vec!(3., 2.))));
    assert_eq!(t.nearest(&Point::from(&vec!(0.,0.))), Some(&Point::from(&vec!(1.,1.5))));
    assert_eq!(t.root.as_ref().map(|r| r.find_min(0)[0]), Some(1.));
    assert_eq!(t.root.as_ref().map(|r| r.find_max(0)[0]), Some(3.));
    assert_eq!(t.find_max(0), Some(&Point::from((3.,2., 0.))));
    assert!(t.find_min(0).unwrap()[0] == 1.);
    (-4..4).for_each(|x|
      (-4..4).for_each(|y| {
        let p = Point::from((x as f32, y as f32, 0.));
        assert_eq!(Some(naive_nearest(&items, &p)), t.nearest(&p));
      }));
  }
  #[test]
  fn gen_test() {
    let mut t = KDTree::new(3);
    let mut r = BadRand::new();
    let cap = 20;
    let points : Vec<_> = (0..256)
      .map(|_| Point::from((r.v(cap) as f32, r.v(cap) as f32, r.v(cap) as f32)))
      .collect();
    points.iter().for_each(|p| t.add(p.clone()));
    assert!(points.iter().all(|p| t.contains(p)));
    (0..256).for_each(|_| {
      let p = Point::from((r.v(cap) as f32, r.v(cap) as f32, r.v(cap) as f32));
      let d1 = naive_nearest(&points, &p).dist(&p);
      let d2 = t.nearest(&p).unwrap().dist(&p);
      assert_eq!(d1, d2);
    });
    (0..3).for_each(|d| {
      assert_eq!(naive_min(&points, d)[d], t.find_min(d).unwrap()[d]);
      assert_eq!(naive_max(&points, d)[d], t.find_max(d).unwrap()[d]);
    });
  }
}
