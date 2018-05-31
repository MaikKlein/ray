pub use cgmath::{Deg, InnerSpace, Rad, Vector3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub dir: Vector3<f32>,
}
impl Ray {
    pub fn new(origin: Vector3<f32>, dir: Vector3<f32>) -> Ray {
        Ray {
            origin,
            dir: dir.normalize(),
        }
    }
}

pub struct Rayhit {
    pub ray: Ray,
    pub t: f32,
}

impl Rayhit {
    pub fn location(&self) -> Vector3<f32> {
        let ray = self.ray;
        ray.origin + ray.dir * self.t
    }
}
