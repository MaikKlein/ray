pub use cgmath::{Deg, InnerSpace, Rad, Vector3};
use material::Material;

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

pub fn random_in_unit_sphere() -> Vector3<f32> {
    use rand::random;
    let v = Vector3::new(random(), random(), random());
    if v.dot(v) >= 1.0 {
        v.normalize() * random()
    } else {
        v
    }
}

use material::{Object, Surface};
pub struct World {
    pub objects: Vec<Object>,
}

impl World {
    pub fn closest_intersect(&self, ray: Ray) -> Option<(&Object, Rayhit)> {
        self.objects
            .iter()
            .filter_map(|object| object.primitive.intersect(ray).map(|rh| (object, rh)))
            .fold(None, |acc, elem| match acc {
                None => Some(elem),
                Some(o) => {
                    if elem.1.dist < o.1.dist {
                        Some(elem)
                    } else {
                        Some(o)
                    }
                }
            })
    }

    pub fn color(&self, ray: Ray) -> Vector3<f32> {
        self.color_impl(ray, 0)
    }

    fn color_impl(&self, ray: Ray, depth: u32) -> Vector3<f32> {
        if depth >= 50 {
            return Vector3::new(0.0, 0.0, 0.0);
        }
        let closest_intersect = self.closest_intersect(ray);
        closest_intersect
            .map(|(object, rayhit)| {
                object
                    .material
                    .scatter(ray, rayhit)
                    .map(|scatter| {
                        let a = scatter.attenuation;
                        let c = self.color_impl(scatter.ray, depth + 1);
                        Vector3::new(a.x * c.x, a.y * c.y, a.z * c.z)
                    })
                    .unwrap_or(Vector3::new(0.0, 0.0, 0.0))
            })
            .unwrap_or_else(|| {
                let t = 0.5 * (ray.dir.y + 1.0);
                (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
            })
    }
}

pub fn reflect(v: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * InnerSpace::dot(v, normal) * normal
}
