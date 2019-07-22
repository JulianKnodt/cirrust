use std::{
  cmp::Ordering,
  f32,
};
use crate::{
  point::Point,
  bounding_box::BoundingBox,
};

#[derive(Debug)]
pub struct KDTree {
  root: Option<KDNode>,
  size: usize,
}

impl KDTree {
  pub fn new() -> Self { KDTree{ root: None, size: 0 } }
  pub fn add(&mut self, v: Point) {
    self.size += 1;
    match &mut self.root {
      None => { self.root.replace(KDNode::new(v, 0)); },
      Some(ref mut r) => r.add(v),
    }
  }
  // This RFC(https://github.com/rust-lang/rust/issues/55300) better be stabilized because it's
  // super convenient
  pub fn from(v: &mut [Point]) -> Self {
    if v.len() == 0 { return KDTree::new(); }
    let d = crate::point::variances(v)
      .iter().enumerate().min_by(|(_,a),(_,b)| a.partial_cmp(&b).unwrap()).unwrap().0;
    let p = v.partition_at_index_by((v.len()-1)/2,
      |a,b| a[d].partial_cmp(&b[d]).unwrap_or(Ordering::Less));
    KDTree{
      root: Some(KDNode::from(p, d)),
      size: v.len(),
    }
  }
	pub fn remove(&mut self, p: &Point) -> bool {
    let did_remove = if self.root.as_ref().map_or(false, |r| &r.item == p) {
      let mut root = self.root.as_mut().unwrap();
      let cmp_dim = root.cmp_dim;
      () == if let Some(r_max) = root.r.as_mut().map(|r| r.find_max(cmp_dim).clone()) {
        assert!(root.remove(&r_max));
        root.item = r_max;
      } else if let Some(l_min) = root.l.as_mut().map(|l| l.find_min(cmp_dim).clone()) {
        assert!(root.remove(&l_min));
        root.item = l_min;
      } else { assert!(self.root.take().unwrap().is_leaf()) }
    } else { self.root.as_mut().map_or(false, |r| r.remove(p)) };
    self.size -= usize::from(did_remove);
    did_remove
  }
  pub fn is_empty(&self) -> bool { self.root.is_none() }
  pub fn contains(&self, v: &Point) -> bool { self.root.as_ref().map_or(false, |r| r.contains(v)) }
  pub fn nearest(&self, v: &Point) -> Option<&Point> { self.root.as_ref().map(|r| r.nearest(v).0) }
  pub fn range<'a>(&self, b: &BoundingBox) -> Vec<Point> {
    let mut buf = vec!();
    self.root.as_ref().map(|r| r.range(b, &mut buf));
    buf
  }
  pub fn find_max(&self, d: usize) -> Option<&Point> { self.root.as_ref().map(|r| r.find_max(d)) }
  pub fn find_min(&self, d: usize) -> Option<&Point> { self.root.as_ref().map(|r| r.find_min(d)) }
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
      let d = crate::point::variances(below)
        .iter().enumerate().min_by(|(_,a),(_,b)| a.partial_cmp(&b).unwrap()).unwrap().0;
      let partition = below.partition_at_index_by(med, |a, b| a[d].partial_cmp(&b[d]).unwrap());
      Some(Box::new(KDNode::from(partition, d)))
    };
    let l = if above.is_empty() { None } else {
      let med = (above.len()-1)/2;
      let d = crate::point::variances(above)
        .iter().enumerate().min_by(|(_,a),(_,b)| a.partial_cmp(&b).unwrap()).unwrap().0;
      let partition = above.partition_at_index_by(med, |a, b| a[d].partial_cmp(&b[d]).unwrap());
      Some(Box::new(KDNode::from(partition, d)))
    };
    KDNode{
      item: median.clone(),
      cmp_dim: cmp_dim,
      l: l, r: r,
    }
  }
  fn add(&mut self, v: Point) {
    let item = match self.item[self.cmp_dim].partial_cmp(&v[self.cmp_dim]) {
      Some(Ordering::Greater) => &mut self.r,
      Some(Ordering::Less) => &mut self.l,
      // randomly select here as it is more resilient if both sides can contain equal values
      // I realize this isn't really random but it can't be counted on
      _ => if (self.cmp_dim % 2) == 0 { &mut self.l } else { &mut self.r },
    };
    let next_dim = (self.cmp_dim + 1) % v.len();
    match item {
      None => assert!(item.replace(Box::new(KDNode::new(v, next_dim))).is_none()),
      Some(ref mut r) => r.add(v),
    };
  }
  fn is_leaf(&self) -> bool { self.l.is_none() && self.r.is_none() }
  fn contains(&self, v: &Point) -> bool {
    if &self.item == v { return true };
    match &self.item[self.cmp_dim].partial_cmp(&v[self.cmp_dim]) {
      Some(Ordering::Greater) => &self.r,
      Some(Ordering::Less) => &self.l,
      // otherwise we check both
      _ => return self.r.as_ref().map_or(false, |r| r.contains(v)) ||
        self.l.as_ref().map_or(false, |l| l.contains(v)),
    }.as_ref()
    .map_or(false, |c| c.contains(v))
  }
  fn remove(&mut self, v: &Point) -> bool {
    let (next, is_r) = match self.item[self.cmp_dim].partial_cmp(&v[self.cmp_dim]) {
      Some(Ordering::Greater) => if self.r.is_none() { return false }
        else { (self.r.as_mut(), true) },
      Some(Ordering::Less) => if self.l.is_none() { return false }
        else { (self.l.as_mut(), false) },
      _ => if self.r.as_ref().map_or(false, |r| &r.item == v) { (self.r.as_mut(), true) }
        else if self.l.as_ref().map_or(false, |l| &l.item == v) { (self.l.as_mut(), false) }
        else if self.r.as_mut().map_or(false, |r| r.remove(v)) { return true }
        else if self.l.as_mut().map_or(false, |l| l.remove(v)) { return true }
        else { return false },
    };
    let next = next.unwrap();
    if &next.item != v { return next.remove(v) };
    let cmp_dim = next.cmp_dim;
    () == if let Some(r_max) = next.r.as_mut().map(|r| r.find_max(cmp_dim).clone()) {
      assert!(next.remove(&r_max));
      next.item = r_max;
    } else if let Some(l_min) = next.l.as_mut().map(|l| l.find_min(cmp_dim).clone()) {
      assert!(next.remove(&l_min));
      next.item = l_min;
    } else if is_r { assert!(self.r.take().unwrap().is_leaf()) }
    else { assert!(self.l.take().unwrap().is_leaf()) }
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
  pub fn range<'a>(&self, b: &BoundingBox, buf: &mut Vec<Point>) {
    if b.contains(&self.item) { buf.push(self.item.clone()); }
    let d = self.cmp_dim;
    match (self.item[d].partial_cmp(&b.min_on(d)), self.item[d].partial_cmp(&b.max_on(d))) {
      (Some(Ordering::Greater), Some(Ordering::Greater)) => {
        self.r.as_ref().map(|r| r.range(b, buf));
      },
      (Some(Ordering::Less), Some(Ordering::Less)) => {
        self.l.as_ref().map(|l| l.range(b, buf));
      },
      (Some(Ordering::Greater), Some(Ordering::Less)) | _  => {
        self.r.as_ref().map(|r| r.range(b, buf));
        self.l.as_ref().map(|l| l.range(b, buf));
      },
    };
  }
  pub fn children(&self) -> std::iter::Chain<
      std::option::Iter<Box<KDNode>>,
      std::option::Iter<Box<KDNode>>
    > {
    self.l.iter().chain(self.r.iter())
  }

  #[cfg(test)]
  fn is_valid(&self) -> bool {
    let right_ok = self.r.as_ref()
      .map_or(true,
        |r| self.item[self.cmp_dim].partial_cmp(&r.item[self.cmp_dim]) != Some(Ordering::Less));
    assert!(right_ok, "Failed on right {:?} {:?}", self.item, self.r.as_ref().unwrap().item);
    let left_ok = self.l.as_ref()
      .map_or(true,
        |l| self.item[self.cmp_dim].partial_cmp(&l.item[self.cmp_dim]) != Some(Ordering::Greater));
    assert!(left_ok, "Failed on left {:?} {:?}", self.item, self.l.as_ref().unwrap().item);
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
    let mut t = KDTree::new();
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

    items.iter().enumerate().for_each(|(i, p)| {
      assert!(t.remove(p));
      assert_eq!(t.size, 4-i-1);
      assert!(t.is_valid());
    });
  }
  #[test]
  fn gen_test() {
    let mut t = KDTree::new();
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
    let mut from = KDTree::from(points.as_mut_slice());
    assert!(t.depth() >= from.depth());
    assert!(from.is_valid());
    points.iter().for_each(|p| {
      assert!(from.remove(p));
      assert!(from.is_valid());
    });
  }

  // TODO add tests for range
}
