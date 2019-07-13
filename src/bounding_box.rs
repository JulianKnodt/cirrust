use crate::point::Point;

#[derive(PartialEq, Debug)]
pub struct BoundingBox {
  dim: usize,
  ll: Point, rr: Point,
}

impl BoundingBox {
  pub fn new(ll: Point, rr:Point) -> Self {
    assert_eq!(ll.len(), rr.len());
    assert!(ll.iter().enumerate().all(|(i,d)| d <= &rr[i]));
    let dim = ll.len();
    BoundingBox{dim, ll, rr}
  }
  pub fn inf(dim: usize) -> Self {
    BoundingBox{
      dim: dim,
      ll: vec![std::f32::NEG_INFINITY; dim],
      rr: vec![std::f32::INFINITY; dim],
    }
  }
  pub fn strictly_contains(&self, p: &Point) -> bool {
    assert_eq!(self.dim, p.len());
    (0..self.dim).fold(true, |prev, d| {
      prev && self.ll[d] < p[d] && p[d] < self.rr[d]
    })
  }
  pub fn contains(&self, p: &Point) -> bool {
    assert_eq!(self.dim, p.len());
    (0..self.dim).all(|d| self.ll[d] <= p[d] && p[d] <= self.rr[d])
  }
  pub fn union(&self, o: &Self) -> Self {
    assert_eq!(self.dim, o.dim);
    BoundingBox{
      dim: self.dim,
      ll: (0..self.dim).map(|d| self.ll[d].min(o.ll[d])).collect(),
      rr: (0..self.dim).map(|d| self.rr[d].max(o.rr[d])).collect(),
    }
  }
  pub fn intersection(&self, o: &Self) -> Self {
    assert_eq!(self.dim, o.dim);
    BoundingBox{
      dim: self.dim,
      ll: (0..self.dim).map(|d| self.ll[d].max(o.ll[d])).collect(),
      rr: (0..self.dim).map(|d| self.rr[d].min(o.rr[d])).collect(),
    }
  }
  pub fn dist(&self, p: &Point) -> f32 {
    (0..self.dim).map(|d| (self.ll[d] - p[d]).max(0.).max(p[d] - self.rr[d]))
      .sum::<f32>()
      .sqrt()
  }
  pub fn expand_to(&mut self, p: &Point) -> bool {
    assert_eq!(self.dim, p.len());
    if self.contains(p) { return false };
    (0..self.dim).for_each(|d| {
      self.ll[d] = self.ll[d].min(p[d]);
      self.rr[d] = self.rr[d].max(p[d]);
    });
    true
  }
  pub fn volume(&self) -> f32 {
    (0..self.dim).map(|d| self.rr[d] - self.ll[d]).product::<f32>()
  }
  pub fn center(&self) -> Point {
    use std::f32;
    (0..self.dim).map(|d| {
      let (a, b) = (self.ll[d], self.rr[d]);
      if a.is_finite() && b.is_finite() { (a+b)/2.0 }
      else if a.is_nan() || b.is_nan() { f32::NAN }
      else if a.is_sign_positive() && b.is_sign_positive() { f32::INFINITY }
      else if a.is_sign_negative() && b.is_sign_negative() { f32::NEG_INFINITY }
      else { 0. }
    }).collect()
  }
  pub fn quadrant(&self, corner: Vec<bool>) -> Self {
    assert_eq!(self.dim, corner.len());
    let (ll, rr) = corner.iter()
      .enumerate()
      .map(|(i,&c)| if c { self.rr[i] } else { self.ll[i] })
      .zip(self.center().iter())
      .map(|(a, &b)| if a < b { (a, b) } else { (b, a) })
      .unzip();
    BoundingBox{
      dim: self.dim,
      ll: ll, rr: rr,
    }
  }
  pub fn surrounds(&self,  o: &Self) -> bool {
    (0..self.dim).all(|d| self.ll[d] < o.ll[d] && self.rr[d] > o.rr[d])
  }
  pub fn on_edge(&self, p: &Point) -> bool {
    assert_eq!(self.dim, p.len());
    (0..self.dim).any(|d| self.ll[d] == p[d] || self.rr[d] == p[d])
  }
}


#[cfg(test)]
mod bounding_box_test {
  use super::BoundingBox;
  fn small_box() -> BoundingBox {
    BoundingBox::new(vec!(-5., -5.), vec!(5.,5.))
  }
  #[test]
  fn test_inf() {
    let inf = BoundingBox::inf(2);
    (0..10).for_each(|v| {
      let v = v as f32;
      assert!(inf.contains(&vec!(v, v)));
      assert!(inf.contains(&vec!(-v, v)));
      assert!(inf.contains(&vec!(v, -v)));
      assert!(inf.contains(&vec!(-v, -v)));
    });
    assert!(inf.surrounds(&small_box()));
    assert_eq!(inf.intersection(&small_box()), small_box());
    assert_eq!(inf.union(&small_box()), inf);
  }
  fn test_small() {
    let bb = small_box();
    (0..=5).for_each(|v| {
      let v = v as f32;
      assert!(bb.contains(&vec!(v, v)));
      assert!(bb.contains(&vec!(-v, v)));
      assert!(bb.contains(&vec!(v, -v)));
      assert!(bb.contains(&vec!(-v, -v)));
    });
    assert!(!bb.contains(&vec!(5.,5.)));
  }
}
