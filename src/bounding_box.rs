use crate::point::{Point};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct BoundingBox {
  ll: Point, rr: Point,
}

impl BoundingBox {
  pub fn new(ll: Point, rr:Point) -> Self {
    assert_eq!(ll.len(), rr.len());
    assert!(ll.iter().enumerate().all(|(i,d)| d <= rr[i]));
    BoundingBox{ll, rr}
  }
  pub fn just(p: &Point) -> Self {
    BoundingBox{
      ll: p.clone(),
      rr: p.clone(),
    }
  }
  pub fn inf(_dim: usize) -> Self {
    BoundingBox{
      ll: Point::from(std::f32::NEG_INFINITY),
      rr: Point::from(std::f32::INFINITY),
    }
  }
  pub fn dim(&self) -> usize { self.ll.len() }
  pub fn strictly_contains(&self, p: &Point) -> bool {
    assert_eq!(self.ll.len(), p.len());
    (0..p.len()).all(|d| self.ll[d] < p[d] && p[d] < self.rr[d])
  }
  pub fn contains(&self, p: &Point) -> bool {
    assert_eq!(self.ll.len(), p.len());
    (0..self.dim()).all(|d| self.ll[d] <= p[d] && p[d] <= self.rr[d])
  }
  pub fn union(&self, o: &Self) -> Self {
    assert_eq!(self.dim(), o.dim());
    BoundingBox{
      ll: (0..self.dim()).map(|d| self.ll[d].min(o.ll[d])).collect(),
      rr: (0..self.dim()).map(|d| self.rr[d].max(o.rr[d])).collect(),
    }
  }
  pub fn intersection(&self, o: &Self) -> Self {
    BoundingBox{
      ll: (0..self.dim()).map(|d| self.ll[d].max(o.ll[d])).collect(),
      rr: (0..self.dim()).map(|d| self.rr[d].min(o.rr[d])).collect(),
    }
  }
  pub fn dist(&self, p: &Point) -> f32 {
    assert_eq!(self.dim(), p.len());
    (0..p.len()).map(|d| (self.ll[d] - p[d]).max(0.).max(p[d] - self.rr[d]))
      .map(|v| v.powi(2))
      .sum::<f32>()
      .sqrt()
  }
  pub fn expand_to(&mut self, p: &Point) -> bool {
    if self.contains(p) { return false };
    (0..p.len()).for_each(|d| {
      self.ll[d] = self.ll[d].min(p[d]);
      self.rr[d] = self.rr[d].max(p[d]);
    });
    true
  }
  pub fn volume(&self) -> f32 {
    (0..self.ll.len()).map(|d| self.rr[d] - self.ll[d]).product::<f32>()
  }
  pub fn center(&self) -> Point {
    use std::f32;
    (0..self.dim()).map(|d| {
      let (a, b) = (self.ll[d], self.rr[d]);
      if a.is_finite() && b.is_finite() { (a+b)/2.0 }
      else if a.is_nan() || b.is_nan() { f32::NAN }
      else if a.is_sign_positive() && b.is_sign_positive() { f32::INFINITY }
      else if a.is_sign_negative() && b.is_sign_negative() { f32::NEG_INFINITY }
      else { 0. }
    }).collect()
  }
  pub fn quadrant(&self, corner: Vec<bool>) -> Self {
    let (ll, rr) = corner.iter()
      .enumerate()
      .map(|(i,&c)| if c { self.rr[i] } else { self.ll[i] })
      .zip(self.center().iter())
      .map(|(a, b)| if a < b { (a, b) } else { (b, a) })
      .unzip();
    BoundingBox{
      ll: ll, rr: rr,
    }
  }
  pub fn surrounds(&self,  o: &Self) -> bool {
    assert_eq!(self.dim(), o.dim());
    (0..self.dim()).all(|d| self.ll[d] <= o.ll[d] && self.rr[d] >= o.rr[d])
  }
  pub fn strictly_surrounds(&self, o: &Self) -> bool {
    assert_eq!(self.dim(), o.dim());
    (0..self.dim()).all(|d| self.ll[d] < o.ll[d] && self.rr[d] > o.rr[d])
  }
  pub fn on_edge(&self, p: &Point) -> bool {
    (0..self.dim()).any(|d| self.ll[d] == p[d] || self.rr[d] == p[d])
  }
  pub fn split_on(&self, d: usize, v: f32) -> (Self, Self) {
    assert!(d < self.dim());
    assert!(self.ll[d] <= v && v <= self.rr[d]);
    let mut mid_ll = self.ll.clone();
    mid_ll[d] = v;
    let mut mid_rr = self.rr.clone();
    mid_rr[d] = v;
    (BoundingBox{
      ll: self.ll.clone(), rr: mid_rr,
    }, BoundingBox{
      ll: mid_ll, rr: self.rr.clone(),
    })
  }
  pub fn overlaps(&self, o: &Self) -> bool {
    assert_eq!(self.dim(), o.dim());
    (0..self.dim()).all(|d| self.rr[d] > o.ll[d] && self.ll[d] < o.rr[d])
  }
}


#[cfg(test)]
mod bounding_box_test {
  use super::BoundingBox;
  use crate::point::Point;
  fn small_box() -> BoundingBox {
    BoundingBox::new(Point::from(&vec!(-5., -5.)), Point::from(&vec!(5.,5.)))
  }
  #[test]
  fn test_inf() {
    let inf = BoundingBox::inf(2);
    (0..10).for_each(|v| {
      let v = v as f32;
      assert!(inf.contains(&Point::from(&vec!(v, v))));
      assert!(inf.contains(&Point::from(&vec!(-v, v))));
      assert!(inf.contains(&Point::from(&vec!(v, -v))));
      assert!(inf.contains(&Point::from(&vec!(-v, -v))));
    });
    assert!(inf.surrounds(&small_box()));
    assert_eq!(inf.intersection(&small_box()), small_box());
    assert_eq!(inf.union(&small_box()), inf);
  }
  #[test]
  fn test_small() {
    let bb = small_box();
    (0..=5).for_each(|v| {
      let v = v as f32;
      assert!(bb.contains(&Point::from(&vec!(v, v))));
      assert!(bb.contains(&Point::from(&vec!(-v, v))));
      assert!(bb.contains(&Point::from(&vec!(v, -v))));
      assert!(bb.contains(&Point::from(&vec!(-v, -v))));
    });
    assert!(!bb.strictly_contains(&Point::from(&vec!(5.,5.))));
    assert!(bb.on_edge(&Point::from(&vec!(5.,5.))));
    assert_eq!(bb.volume(), 0.);
  }

  #[test]
  fn test_dist() {
    let origin: Point = Default::default();
    let v = Point::from((5.,0.,0.));
    let mut bb_origin = BoundingBox::just(&origin);
    assert_eq!(bb_origin.dist(&v), 5.0);
    bb_origin.expand_to(&Point::from((0., 100., 0.)));
    assert_eq!(bb_origin.dist(&v), 5.0);
    bb_origin.expand_to(&Point::from((0., 0., 100.)));
    assert_eq!(bb_origin.dist(&v), 5.0);
    bb_origin.expand_to(&Point::from((3., 0., 0.)));
    assert_eq!(bb_origin.dist(&v), 2.0);
    bb_origin.expand_to(&Point::from(100.));
    assert_eq!(bb_origin.dist(&v), 0.);
  }
  #[test]
  fn test_expand() {
    let mut empty = BoundingBox::just(&Default::default());
    assert!(empty.expand_to(&Point::from(5.)));
    assert!(!empty.contains(&Point::from(-5.)));
  }
}

