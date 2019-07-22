use std::{
  iter::{Iterator, Extend},
  ops::{Index, IndexMut},
  convert::{From},
};

// Temporary representation of 3d point, might do more dimenstions later, idk
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Point(f32, f32, f32);

pub fn l2norm(a: &Point, b: &Point) -> f32 {
  assert_eq!(a.len(), b.len());
  (0..a.len())
    .map(|d| (a[d] - b[d]).powi(2))
    .sum::<f32>()
    .powf(0.5)
}

pub fn l1norm(a: &Point, b: &Point) -> f32 {
  assert_eq!(a.len(), b.len());
  (0..a.len())
    .map(|d| (a[d] - b[d]).abs())
    .sum::<f32>()
}

impl Index<usize> for Point {
  type Output = f32;
  fn index(&self, i: usize) -> &f32 {
    match i {
      0 => &self.0,
      1 => &self.1,
      2 => &self.2,
      _ => panic!("Index out of bounds"),
    }
  }
}

impl IndexMut<usize> for Point {
  fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Self::Output {
    match i {
      0 => &mut self.0,
      1 => &mut self.1,
      2 => &mut self.2,
      _ => panic!("Index out of bounds"),
    }
  }
}

impl std::iter::FromIterator<f32> for Point {
  fn from_iter<I: IntoIterator<Item=f32>>(iter: I) -> Self {
    let mut i = iter.into_iter();
    Point(i.next().unwrap(), i.next().unwrap(), i.next().unwrap())
  }
}

impl Point {
  pub fn len(&self) -> usize { 3 }
  pub fn iter(&self) -> Iter { Iter(0, &self) }
  pub fn dist(&self, o: &Self) -> f32 { l2norm(self, o) }
  pub fn get(&self, d: usize) -> Option<f32> {
    match d {
      0 | 1 | 2 => Some(self[d]),
      _ => None,
    }
  }
}

pub fn variances(p: &[Point]) -> Vec<f32> {
  if p.is_empty() { return vec!() };
  (0..p[0].len())
    .map(|d| crate::util::variance(p.iter().map(|p| p[d])))
    .collect()
}

#[derive(Clone, Copy, Debug)]
pub struct Iter<'a>(i32, &'a Point);

impl Iterator for Iter<'_> {
  type Item = f32;
  fn next(&mut self) -> Option<Self::Item> {
    let out = match self.0 {
      0 => Some(self.1[0]),
      1 => Some(self.1[1]),
      2 => Some(self.1[2]),
      _ => None
    };
    self.0 += 1;
    out
  }
}

impl Extend<f32> for Point {
  fn extend<I: IntoIterator<Item=f32>>(&mut self, iter: I) {
    let mut i = iter.into_iter();
    self.0 = i.next().unwrap();
    self.1 = i.next().unwrap();
    self.2 = i.next().unwrap();
  }
}

impl From<&Vec<f32>> for Point {
  fn from(v: &Vec<f32>) -> Self {
    Point(
      v.get(0).copied().unwrap_or(0.),
      v.get(1).copied().unwrap_or(0.),
      v.get(2).copied().unwrap_or(0.),
    )
  }
}

impl From<f32> for Point {
  fn from(v: f32) -> Self { Point(v,v,v) }
}

impl From<(f32, f32, f32)> for Point {
  fn from(v: (f32, f32,f32)) -> Self { Point(v.0, v.1, v.2) }
}

#[cfg(test)]
mod tests {
  use super::Point;
  #[test]
  fn test_point() {
    let p: Point = Default::default();
    assert_eq!(p.0, p.1);
    assert_eq!(p.0, p.2);
    assert_eq!(p.0, 0.);
    assert!(p.iter().all(|v| v == 0.));
    assert_eq!(p.iter().count(), 3);
  }

  #[test]
  fn test_dist() {
    let origin: Point = Default::default();
    let v = Point::from((3., 4., 0.));
    assert_eq!(origin.dist(&v), 5.);
  }
}
