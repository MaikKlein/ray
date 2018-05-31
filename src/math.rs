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

pub fn random_in_unit_sphere() -> Vector3<f32> {
    use rand::random;
    let v = Vector3::new(random(), random(), random());
    if v.dot(v) >= 1.0 {
        v.normalize() * random()
    } else {
        v
    }
}

use primitive::Sphere;
pub struct World {
    pub objects: Vec<Sphere>,
}
impl World {
    pub fn color(&self, ray: Ray) -> Vector3<f32> {
        let hit = self.intersect(ray);
        if let Some(rayhit) = hit {
            let target = rayhit.position + rayhit.normal + random_in_unit_sphere();
            0.5 * self.color(Ray::new(rayhit.position, target - rayhit.position))
        } else {
            let t = 0.5 * (ray.dir.y + 1.0);
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
        }
    }
}
impl Intersect for World {
    fn intersect(&self, ray: Ray) -> Option<Rayhit> {
        self.objects.iter().filter_map(|s| s.intersect(ray)).nth(0)
    }
}
