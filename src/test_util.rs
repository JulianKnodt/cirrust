// Just a bad implementation of random for testing
// Didn't want to bring in the crate just for that
pub struct BadRand(i64);
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
  pub fn i64(&mut self, m: i64) -> i64 {
    self.0 = (self.0 * A + C) % M;
    return self.0 % m;
  }
  pub fn f64(&mut self) -> f64 {
    self.0 = (self.0 * A + C) % M;
    unsafe {
      std::mem::transmute::<i64, f64>(self.0)
    }
  }
}
