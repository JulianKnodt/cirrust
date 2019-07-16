use std::{
  iter::Iterator,
};

// Temporary representation of 3d point
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

impl std::ops::Index<usize> for Point {
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

impl std::ops::IndexMut<usize> for Point {
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
  pub fn from_vec(v: Vec<f32>) -> Self {
    Point(
      v.get(0).copied().unwrap_or(0.),
      v.get(1).copied().unwrap_or(0.),
      v.get(2).copied().unwrap_or(0.),
    )
  }
  pub fn len(&self) -> usize { 3 }
  pub fn iter(&self) -> Iter { Iter(0, &self) }
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

#[derive(Default, Clone, Copy, Debug)]
pub(crate) struct OptionalPoint(Option<f32>, Option<f32>, Option<f32>);
impl OptionalPoint {
  pub fn unwrap(&self) -> Point {
    Point(self.0.unwrap(), self.1.unwrap(), self.2.unwrap())
  }
}

impl std::iter::Extend<f32> for OptionalPoint {
  fn extend<I: IntoIterator<Item=f32>>(&mut self, iter: I) {
    let mut i = iter.into_iter();
    assert!(self.0.replace(i.next().unwrap()).is_none());
    assert!(self.1.replace(i.next().unwrap()).is_none());
    assert!(self.2.replace(i.next().unwrap()).is_none());
  }
}
