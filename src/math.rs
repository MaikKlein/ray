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

    pub fn position(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.dir
    }
}

pub struct Rayhit {
    pub dist: f32,
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
}

pub trait Intersect {
    fn intersect(&self, ray: Ray) -> Option<Rayhit>;
}
