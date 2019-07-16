use crate::{
  bounding_box::BoundingBox,
  point::Point,
};

pub trait Bounded {
  // return the bounding box for this object
  fn bounds(&self) -> BoundingBox;
}

impl Bounded for BoundingBox {
  fn bounds(&self) -> Self { self.clone() }
}

impl Bounded for Point {
  fn bounds(&self) -> BoundingBox { BoundingBox::just(self) }
}

