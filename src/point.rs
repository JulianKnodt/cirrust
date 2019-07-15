// Placeholder type for finite sized Point, works for now but can be replaced later
pub type Point = Vec<f32>;

pub fn l2norm(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
  assert_eq!(a.len(), b.len());
  (0..a.len())
    .map(|d| (a[d] - b[d]).powi(2))
    .sum::<f32>()
    .powf(0.5)
}

pub fn l1norm(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
  assert_eq!(a.len(), b.len());
  (0..a.len())
    .map(|d| (a[d] - b[d]).abs())
    .sum::<f32>()
}

// Takes the average of a list of points
// there are other ways of computing this more efficiently
pub fn median(of: Vec<Point>) -> Point {
  assert!(!of.is_empty());
  (0..of[0].len()).map(|d| {
    let mut items : Vec<_> = of.iter()
      .map(|p| p[d])
      .filter(|v| v.is_finite())
      .collect();
    items.sort_unstable_by(|x,y| x.partial_cmp(y).unwrap());
    items[of.len() / 2]
  }).collect()
}
