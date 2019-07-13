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

