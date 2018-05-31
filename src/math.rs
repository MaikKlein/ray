pub use cgmath::{Vector3, InnerSpace, Deg, Rad};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub dir: Vector3<f32>,
}
impl Ray {
    pub fn new(origin: Vector3<f32>, dir: Vector3<f32>) -> Ray {
        Ray { origin, dir }
    }
}
