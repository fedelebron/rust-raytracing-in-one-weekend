// A bounding volume hierarchy.
use crate::aabb::*;
use crate::object::*;
use crate::ray::*;
use crate::vec3::*;
use rand::Rng;
use std::cmp::Ordering;
use std::sync::Arc;

type T = f32;

pub struct BVHNode {
  left: Option<Box<Object>>,
  right: Option<Box<Object>>,
  bounding_box: BoundingBox,
}

impl BVHNode {
  pub fn new() -> BVHNode {
    BVHNode {
      left: None,
      right: None,
      bounding_box: BoundingBox::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(0.0, 0.0, 0.0),
      ),
    }
  }

  pub fn new_from_objects(
    objects: &mut [Option<Box<Object>>],
    time0: T,
    time1: T,
  ) -> BVHNode {
    let mut rng = rand::thread_rng();
    let axis: usize = rng.gen_range(0..3);
    let comparator = |x: &Option<Box<Object>>,
                      y: &Option<Box<Object>>| {
      compare_by_dim(&**(x.as_ref().unwrap()), &**(y.as_ref().unwrap()), time0, time1, axis)
    };
    let mut node = BVHNode::new();
    let n = objects.len();
    if n == 1 {
      node.left = objects[0].take();
      node.right = None;
    } else if n == 2 {
      if comparator(&objects[0], &objects[1]) == Ordering::Less {
        node.left = objects[0].take();
        node.right = objects[1].take();
      } else {
        node.left = objects[1].take();
        node.right = objects[0].take();
      }
    } else {
      objects.sort_by(comparator);
      let mid = n / 2;
      node.left = Some(Box::new(Object::BVHNodeType(BVHNode::new_from_objects(
        &mut objects[0..mid],
        time0,
        time1,
      ))));
      node.right = Some(Box::new(Object::BVHNodeType(BVHNode::new_from_objects(
        &mut objects[mid..n],
        time0,
        time1,
      ))));
    }
    let mut left_bb = node
      .left
      .as_ref()
      .unwrap()
      .bounding_box(time0, time1)
      .unwrap();
    match node.right {
      None => { node.bounding_box = left_bb; }
      Some (ref right_node) => {
        let right_bb = right_node.bounding_box(time0, time1).unwrap();
        node.bounding_box = BoundingBox::surrounding_box(&left_bb, &right_bb);
      }
    }
    
    node
  }
}

impl BVHNode {
  pub fn hit(&self, t_min: T, t_max: T, ray: &Ray) -> Option<HitResult> {
    if !self.bounding_box.hit(t_min, t_max, ray) {
      return None;
    }
    
    let left_node = self.left.as_ref().unwrap();

    let left_hit = left_node.hit(t_min, t_max, ray);
    match left_hit {
      None => {
          self.right.as_ref().and_then(|node| node.hit(t_min, t_max, ray)) }
      Some(ref hrl) => {
        match self.right {
          None => { left_hit },
          Some(ref right_node) => { right_node.hit(t_min, hrl.t, ray).or(left_hit) }
        }
      }
    }
  }
  pub fn bounding_box(&self, _time0: T, _time1: T) -> Option<BoundingBox> {
    Some(self.bounding_box)
  }
}

fn compare_by_dim(
  a: &Object,
  b: &Object,
  time0: T,
  time1: T,
  axis: usize,
) -> Ordering {
  // Note that comparing at time 0 here doesn't change much, it just means objects will fall into
  // different bounding boxes that may not be as efficient as possible at a given time, but the
  // bounding boxes will still be correctly computed.
  let bb_a = a.bounding_box(time0, time1);
  let bb_b = b.bounding_box(time0, time1);
  match (bb_a, bb_b) {
    (Some(x), Some(y)) => x.less_than_by_dim(&y, axis),
    _ => panic!(
      "Unable to compute bounding boxes for an object in the BVH: {:?}",
      (bb_a, bb_b)
    ),
  }
}
