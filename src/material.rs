use math::{random_in_unit_sphere, reflect, InnerSpace, Intersect, Ray, Rayhit, Vector3};
use primitive::Primitive;

pub trait Surface {
    fn scatter(&self, ray_in: Ray, ray: Rayhit) -> Option<Scatter>;
}

#[derive(Debug, Copy, Clone)]
pub struct Scatter {
    pub attenuation: Vector3<f32>,
    pub ray: Ray,
}

// impl Surface for Object {
//     fn scatter(&self, ray_in: Ray, rayhit: Rayhit) -> Option<Scatter> {
//         self.material.scatter(ray_in, rayhit)
//     }
// }

// impl Intersect for Object {
//     fn intersect(&self, ray: Ray) -> Option<Rayhit> {
//         self.primitive.intersect(ray)
//     }
// }

pub struct Object {
    pub primitive: Primitive,
    pub material: Material,
}

pub enum Material {
    Lambert(Lambert),
    Metal(Metal),
}

impl Material {
    pub fn scatter(&self, ray_in: Ray, rayhit: Rayhit) -> Option<Scatter> {
        match self {
            Material::Lambert(lambert) => lambert.scatter(ray_in, rayhit),
            Material::Metal(metal) => metal.scatter(ray_in, rayhit),
        }
    }
}

pub struct Lambert {
    pub albedo: Vector3<f32>,
}

impl Lambert {
    pub fn scatter(&self, _: Ray, rayhit: Rayhit) -> Option<Scatter> {
        let target = rayhit.position + rayhit.normal + random_in_unit_sphere();
        let ray = Ray::new(rayhit.position, target - rayhit.position);
        Some(Scatter {
            attenuation: self.albedo,
            ray,
        })
    }
}

pub struct Metal {
    pub albedo: Vector3<f32>,
    pub fuzz: f32,
}
impl Metal {
    pub fn scatter(&self, ray_in: Ray, rayhit: Rayhit) -> Option<Scatter> {
        let reflect = reflect(ray_in.dir, rayhit.normal) + self.fuzz * random_in_unit_sphere();
        let ray = Ray::new(rayhit.position, reflect);
        if InnerSpace::dot(ray.dir, rayhit.normal) > 0.0 {
            Some(Scatter {
                attenuation: self.albedo,
                ray,
            })
        } else {
            None
        }
    }
}
