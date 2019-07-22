// Finds average of an iterator f32s
// Caution when using on large streams
pub fn average<I>(v: I) -> f32 where I: IntoIterator<Item = f32> {
  v.into_iter().enumerate().fold(0., |prev, (n, c)| {
    let n = n as f32;
    (n * prev + c)/(n + 1.)
  })
}

// Finds variance of an iterator f32s
pub fn variance<I>(v: I) -> f32 where I: IntoIterator<Item = f32> + Clone {
  let iter = v.clone().into_iter();
  let avg = average(v);
  average(iter.map(|f| (f - avg).powi(2)))
}

#[cfg(test)]
mod average_variance_tests {
  use super::*;
  #[test]
  fn large_constant() {
    vec!(1, 10, 100, 1000, 10000, 100000).into_iter().for_each(|n| {
      assert_eq!(average(Some(5.).into_iter().cycle().take(n)), 5.);
      assert_eq!(variance(Some(5.).into_iter().cycle().take(n)), 0.);
    });
  }
  #[test]
  fn linear() {
    vec!(1, 10, 100, 1000).into_iter().for_each(|n| {
      // seems to fail for larger numbers due to numerical instability but still close
      assert_eq!(average((0..=n).map(|i| i as f32).rev()), (n as f32)/2.);
    });
  }
}


// The below could be abstracted over orderable types, but not really worth it yet

// sorts and returns median of v
// will return the lower of the middle two if v is of even length
pub fn sorted_median(v: &mut [f32]) -> f32 {
  v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
  v[(v.len()-1)/2]
}

pub fn pivot(v: &mut [f32]) -> f32 {
  // just default for smaller arrays  because it's not worth it.
  if v.len() < 20 { return sorted_median(v) };
  let mut medians : Vec<_> = v.chunks_mut(10)
    .map(|mut chunk| sorted_median(&mut chunk))
    .collect();
  sorted_median(&mut medians.as_mut_slice())
}

pub fn quickselect(v: &mut [f32], k: usize) -> f32 {
  if v.len() == 1 {
    assert_eq!(k, 0);
    return v[0]
  }
  let p = pivot(v);
  use std::cmp::Ordering;
  let (mut g, mut l, mut eq) = (vec!(), vec!(), vec!());
  v.iter() .for_each(|o| match o.partial_cmp(&p).unwrap() {
    Ordering::Equal => eq.push(*o),
    Ordering::Greater => g.push(*o),
    Ordering::Less => l.push(*o),
  });
  if k < l.len() { quickselect(l.as_mut_slice(), k) }
  else if k < l.len() + eq.len() { p }
  else { quickselect(g.as_mut_slice(), k - l.len() - eq.len()) }
}

pub fn median(v: &mut [f32]) -> f32 { quickselect(v, (v.len() - 1)/2) }

#[cfg(test)]
mod median_tests {
  use super::*;
  #[test]
  fn naive() {
    let mut v = vec!(0.,1.,2.,3.,4.);
    assert_eq!(sorted_median(&mut v), 2.);
    assert_eq!(median(&mut v), 2.);
    let mut even = vec!(0.,1.,2.,3.,4.,5.);
    assert_eq!(sorted_median(&mut even), 2.);
    assert_eq!(median(&mut even), 2.);
  }
  fn test_quickselect() {
    let mut v : Vec<_> = (0..100).map(|i| i as f32).collect();
    (0..100).for_each(|i| {
      assert_eq!(quickselect(v.as_mut_slice(), i), i as f32);
    });
  }
}

