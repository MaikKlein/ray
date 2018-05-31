use math::{InnerSpace, Ray, Rayhit, Vector3};
pub struct Sphere {
    pub origin: Vector3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn new(origin: Vector3<f32>, radius: f32) -> Sphere {
        Sphere { origin, radius }
    }

    /// Returns the closest interection.
    /// The formula below is derived from the implicit surface of a sphere.
    /// If the disremninant is negative, then we have an imaginary solution and
    /// we return None.
    /// If t is negative, this means that the intersection is behind the camera
    /// and we return None
    /// If t is positive we return the smallest t because this is the closest
    /// intersection.
    pub fn intersect(&self, ray: Ray) -> Option<Rayhit> {
        let oc = ray.origin - self.origin;
        let a = InnerSpace::dot(ray.dir, ray.dir);
        let b = 2.0 * InnerSpace::dot(ray.dir, oc);
        let c = InnerSpace::dot(oc, oc) - self.radius * self.radius;
        let discr = b * b - 4.0 * a * c;
        let t = if discr < 0.0 {
            -1.0
        } else {
            let two_a = 2.0 * a;
            let t1 = (-b - discr.sqrt()) / two_a;
            let t2 = (-b + discr.sqrt()) / two_a;
            f32::min(t1, t2)
        };
        if t < 0.0 {
            None
        } else {
            Some(Rayhit { ray, t })
        }
    }
}