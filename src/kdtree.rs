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
  pub fn add(&mut self, v: Point) {
    self.size += 1;
    match &mut self.root {
      None => { self.root.replace(KDNode::new(v, 0)); },
      Some(ref mut r) => r.add(v, self.dims),
    }
  }
  // This RFC(https://github.com/rust-lang/rust/issues/55300) better be stabilized because it's
  // super convenient
  pub fn from(v: &mut [Point]) -> Self {
    let d = 0;
    let med = (v.len()-1)/2;
    let partitions = v.partition_at_index_by(med, |a, b| match a[d].partial_cmp(&b[d]) {
      Some(Ordering::Greater) => Ordering::Greater,
      _ => Ordering::Less,
    });
    let dims = partitions.1.len();
    KDTree{
      dims: dims,
      root: Some(KDNode::from(partitions, d)),
      size: v.len(),
    }
  }
  pub fn is_empty(&self) -> bool { self.root.is_some() }
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
  pub fn depth(&self) -> usize { self.root.as_ref().map_or(0, |r| r.depth()) }
  #[cfg(test)]
  fn is_valid(&self) -> bool {
    assert!(self.root.as_ref().map_or(true, |r| r.is_valid()));
    assert_eq!(self.size, self.root.as_ref().map_or(0, |r| r.count()));
    true
  }
}

#[derive(Debug)]
pub struct KDNode {
  item: Point,
  cmp_dim: usize,
  // l is lesser and equal
  l: Option<Box<KDNode>>,
  // right is greater and equal
  r: Option<Box<KDNode>>,
}

impl KDNode {
  fn new(v: Point, dim: usize) -> Self {
    KDNode{
      item: v, cmp_dim: dim,
      l: None, r: None,
    }
  }
  fn from(partition: (&mut [Point], &mut Point, &mut [Point]), cmp_dim: usize) -> Self {
    let (below, median, above) = partition;
    let r = if below.is_empty() { None } else {
      let med = (below.len()-1)/2;
      let d = (cmp_dim + 1) % median.len();
      let partition = below.partition_at_index_by(med, |a, b| a[d].partial_cmp(&b[d]).unwrap());
      Some(Box::new(KDNode::from(partition, d)))
    };
    let l = if above.is_empty() { None } else {
      let med = (above.len()-1)/2;
      let d = (cmp_dim + 1) % median.len();
      let partition = above.partition_at_index_by(med, |a, b| a[d].partial_cmp(&b[d]).unwrap());
      Some(Box::new(KDNode::from(partition, d)))
    };
    KDNode{
      item: median.clone(),
      cmp_dim: cmp_dim,
      l: l, r: r,
    }
  }
  fn add(&mut self, v: Point, dims: usize) {
    let item = match self.item[self.cmp_dim].partial_cmp(&v[self.cmp_dim]) {
      Some(Ordering::Greater) => &mut self.r,
      Some(Ordering::Less) => &mut self.l,
      // randomly select here as it is more resilient if both sides can contain equal values
      _ => unimplemented!(),
    };
    let next_dim = (self.cmp_dim + 1) % dims;
    match item {
      None => assert!(item.replace(Box::new(KDNode::new(v, next_dim))).is_none()),
      Some(ref mut r) => r.add(v, dims),
    };
  }
  fn is_leaf(&self) -> bool { self.l.is_none() && self.r.is_none() }
  fn contains(&self, v: &Point) -> bool {
    if &self.item == v { return true };
    match &self.item[self.cmp_dim].partial_cmp(&v[self.cmp_dim]) {
      Some(Ordering::Greater) => &self.r,
      Some(Ordering::Less) => &self.r,
      // otherwise we check both
      _ => return self.r.as_ref().map_or(false, |r| r.contains(v)) ||
        self.l.as_ref().map_or(false, |l| l.contains(v)),
    }.as_ref()
    .map_or(false, |c| c.contains(v))
  }
  fn find_min(&self, d: usize) -> &Point {
    let l = self.l.as_ref().filter(|_| self.cmp_dim != d).map(|l| l.find_min(d));
    let r = self.r.as_ref().map(|r| r.find_min(d));
    match (r, l) {
      (None, None) => &self.item,
      (Some(v), None) | (None, Some(v)) => if self.item[d] < v[d] { &self.item }
        else { v },
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
      (Some(v), None) | (None, Some(v)) => if self.item[d] > v[d] { &self.item }
        else { v },
      (Some(r), Some(l)) => if r[d] > l[d] { r }
        else if self.item[d] > l[d] { &self.item }
        else { l },
    }
  }
  fn nearest(&self, v: &Point) -> (&Point, f32) {
    let self_dist = self.item.dist(v);
    let (close, far) = match self.item[self.cmp_dim].partial_cmp(&v[self.cmp_dim]) {
      Some(Ordering::Greater) => (&self.r, &self.l),
      Some(Ordering::Less) | _ => (&self.l, &self.r),
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

  pub fn children(&self) -> std::iter::Chain<
      std::option::Iter<Box<KDNode>>,
      std::option::Iter<Box<KDNode>>
    > {
    self.l.iter().chain(self.r.iter())
  }

  #[cfg(test)]
  fn is_valid(&self) -> bool {
    self.r.as_ref()
      .map(|r| assert!(self.is_r(&r.item), "\n{:?} =bad r> {:?} on {}",
        self.item, r.item, self.cmp_dim));
    self.l.as_ref()
      .map(|l| assert!(!self.is_r(&l.item), "\n{:?} =bad l> {:?}", self.item, l.item));
    self.r.as_ref().map(|r| assert!(r.is_valid()));
    self.l.as_ref().map(|l| assert!(l.is_valid()));
    true
  }

  #[cfg(test)]
  fn count(&self) -> usize {
    1 + self.r.as_ref().map_or(0, |r| r.count()) + self.l.as_ref().map_or(0, |l| l.count())
  }

  pub fn depth(&self) -> usize {
    1 + self.r.as_ref()
      .map_or(0, |r| r.depth())
      .max(self.l.as_ref().map_or(0, |l| l.depth()))
  }
}

#[cfg(test)]
mod kdtree_test {
  use crate::kdtree::KDTree;
  use crate::point::Point;
  use crate::test_util::BadRand;
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
  #[test]
  fn all_test() {
    let mut t = KDTree::new(2);
    let items: Vec<_> = vec!((2.,3.), (3.,2.), (1.,1.5), (1.,2.)).iter()
      .map(|&(a,b)| Point::from((a,b,0.)))
      .collect();
    items.iter().for_each(|p| t.add(p.clone()));
    assert_eq!(t.size(), 4);
    assert!(t.is_valid());
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
    assert!(t.is_valid());
    assert_eq!(t.size, 4);
  }
  #[test]
  fn gen_test() {
    let mut t = KDTree::new(3);
    let mut r = BadRand::new();
    let cap = 20;
    let num_points = 256;
    let mut points : Vec<_> = (0..num_points)
      .map(|_| Point::from((r.i64(cap) as f32, r.i64(cap) as f32, r.i64(cap) as f32)))
      .collect();
    points.iter().for_each(|p| t.add(p.clone()));
    assert!(t.is_valid());
    assert_eq!(num_points, t.root.as_ref().unwrap().count());
    assert_eq!(num_points, t.size);
    assert!(points.iter().all(|p| t.contains(p)));
    (0..256).for_each(|_| {
      let p = Point::from((r.i64(cap) as f32, r.i64(cap) as f32, r.i64(cap) as f32));
      let d1 = naive_nearest(&points, &p).dist(&p);
      let d2 = t.nearest(&p).unwrap().dist(&p);
      assert_eq!(d1, d2);
    });
    (0..3).for_each(|d| {
      assert_eq!(naive_min(&points, d)[d], t.find_min(d).unwrap()[d]);
      assert_eq!(naive_max(&points, d)[d], t.find_max(d).unwrap()[d]);
    });
    assert!(t.is_valid());
    let from = KDTree::from(points.as_mut_slice());
    assert!(t.depth() >= from.depth());
    assert!(from.is_valid());
  }
}
