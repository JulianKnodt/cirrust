#![allow(dead_code)]
#![feature(ptr_internals, slice_partition_at_index)]

pub mod point;
pub mod kdtree;
pub mod bounding_box;
pub mod bounded;
pub mod iters;
pub(crate) mod util;
// pub mod rtree;
//pub mod mesh;

#[cfg(test)]
pub(crate) mod test_util;
