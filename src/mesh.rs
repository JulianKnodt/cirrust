use crate::point::Point;
use std::io::{self, BufReader, BufRead};

pub struct Mesh {
  pts: Vec<Point>,
  textures: Vec<Point>,
  normals: Vec<Point>,
  polygons: Vec<Vec<(usize, Option<usize>, Option<usize>)>>,
  dims: usize,
}

impl Mesh {
  fn new(dims: usize) -> Self {
    assert!(dims > 0);
    Mesh{
      pts: vec!(),
      textures: vec!(),
      normals: vec!(),
      polygons: vec!(),
      dims: dims,
    }
  }
  fn from_obj_file(name: &str) -> io::Result<Self> {
    let b = BufReader::new(std::fs::File::open(name)?);
    let mut t = Self::new(3);
    for line in b.lines() {
      let line = line?.clone();
      let mut tokens = line.split_whitespace();
      match tokens.next() {
        None => continue,
        Some("v") => {
          let v : Vec<_> = tokens.map(|t| t.parse::<f32>().unwrap()).collect();
          assert!(v.len() == 3 || v.len() == 4);
          t.pts.push(v);
        },
        Some("f") => {
          let face : Vec<_> = tokens.map(|t| {
            let mut coord = (0, None, None);
            t.split("/")
            .enumerate()
            .for_each(|(i, t)| match i {
              0 => coord.0 = t.parse::<usize>().unwrap(),
              1 => coord.1 = Some(t.parse::<usize>().unwrap()),
              2 => coord.2 = Some(t.parse::<usize>().unwrap()),
              _ => panic!("Got extra token while parsing"),
            });
            coord
          }).collect();
          t.polygons.push(face);
        },
        Some(v) => panic!("Unimplemented parsing in obj for {}", v),
      }
    }
    Ok(t)
  }
}
