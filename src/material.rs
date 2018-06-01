use math::{random_in_unit_sphere, Intersect, Ray, Rayhit, Vector3};
use primitive::Primitive;

pub trait Surface {
    fn scatter(&self, ray: Rayhit) -> Scatter;
}

#[derive(Debug, Copy, Clone)]
pub struct Scatter {
    pub attenuation: Vector3<f32>,
    pub ray: Ray,
}

impl Surface for Object {
    fn scatter(&self, rayhit: Rayhit) -> Scatter {
        self.material.scatter(rayhit)
    }
}

impl Intersect for Object {
    fn intersect(&self, ray: Ray) -> Option<Rayhit> {
        self.primitive.intersect(ray)
    }
}

pub struct Object {
    pub primitive: Primitive,
    pub material: Material,
}

pub enum Material {
    Lambert(Lambert),
}

impl Material {
    pub fn scatter(&self, rayhit: Rayhit) -> Scatter {
        match self {
            Material::Lambert(lambert) => lambert.scatter(rayhit),
        }
    }
}

pub struct Lambert {
    pub albedo: Vector3<f32>,
}

impl Lambert {
    pub fn scatter(&self, rayhit: Rayhit) -> Scatter {
        let target = rayhit.position + rayhit.normal + random_in_unit_sphere();
        let ray = Ray::new(rayhit.position, target - rayhit.position);
        Scatter {
            attenuation: self.albedo,
            ray,
        }
    }
}

pub struct Metal {}
